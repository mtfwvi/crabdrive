use crate::storage::file::file_repository::FileRepository;
use crate::storage::file::model::TransferSessionId;
use crate::storage::node::persistence::model::encrypted_metadata::EncryptedMetadata;
use crate::storage::node::persistence::model::node_entity::NodeEntity;
use crate::storage::node::persistence::node_repository::NodeRepository;
use crate::storage::revision::persistence::model::revision_entity::RevisionEntity;
use crate::storage::revision::persistence::revision_repository::RevisionRepository;
use anyhow::{Result, anyhow};
use chrono::Utc;
use crabdrive_common::iv::IV;
use crabdrive_common::storage::{ChunkIndex, NodeId, NodeType};
use crabdrive_common::uuid::UUID;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct TemporaryUploadStorage {
    nodes: HashMap<NodeId, UploadSession>,
}

struct UploadSession {
    node: NodeEntity,
    revision: RevisionEntity,
    chunk_count: ChunkIndex,
    session_id: TransferSessionId,
    chunks_uploaded: ChunkIndex,
}

impl TemporaryUploadStorage {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }
}

pub struct FileOperations<N: NodeRepository, R: RevisionRepository, F: FileRepository> {
    node_repo: Arc<N>,
    revision_repo: Arc<R>,
    file_repo: Arc<F>,
    temp_storage: Arc<Mutex<TemporaryUploadStorage>>,
}

impl<N: NodeRepository, R: RevisionRepository, F: FileRepository> FileOperations<N, R, F> {
    pub fn new(node_repo: Arc<N>, revision_repo: Arc<R>, file_repo: Arc<F>) -> Self {
        Self {
            node_repo,
            revision_repo,
            file_repo,
            temp_storage: Arc::new(Mutex::new(TemporaryUploadStorage::new())),
        }
    }

    pub fn create_file(
        &self,
        parent_id: NodeId,
        parent_metadata: EncryptedMetadata,
        node_metadata: EncryptedMetadata,
        chunk_count: ChunkIndex,
        owner: UUID,
        iv: IV,
    ) -> Result<NodeId> {
        let parent_node = self.node_repo.get_node(parent_id)?;
        let mut updated_parent = parent_node;
        updated_parent.metadata = parent_metadata;
        updated_parent.metadata_change_counter += 1;
        self.node_repo.update_node(updated_parent)?;

        let node = self.node_repo.create_node(
            Some(parent_id),
            node_metadata,
            owner,
            NodeType::File,
        )?;

        let node_id = node.id;

        let revision = self.revision_repo.create_revision(
            node_id,
            Utc::now().naive_utc(),
            iv,
        )?;

        let session_id = self.file_repo.start_transfer(node_id.to_string())?;

        let mut temp = self.temp_storage.lock().unwrap();
        temp.nodes.insert(
            node_id,
            UploadSession {
                node,
                revision,
                chunk_count,
                session_id,
                chunks_uploaded: 0,
            },
        );

        Ok(node_id)
    }

    pub fn upload_chunk(
        &self,
        node_id: NodeId,
        chunk: crate::storage::file::model::FileChunk,
    ) -> Result<()> {
        let mut temp = self.temp_storage.lock().unwrap();
        let session = temp
            .nodes
            .get_mut(&node_id)
            .ok_or_else(|| anyhow!("Node {} not found in temporary storage", node_id))?;

        let session_id = session.session_id.clone();
        
        self.file_repo.write_chunk(&session_id, chunk)?;
        
        session.chunks_uploaded += 1;

        Ok(())
    }

    pub fn finish_upload(&self, node_id: NodeId) -> Result<()> {
        let mut temp = self.temp_storage.lock().unwrap();
        let session = temp
            .nodes
            .remove(&node_id)
            .ok_or_else(|| anyhow!("Node {} not found in temporary storage", node_id))?;
        drop(temp);

        if session.chunks_uploaded != session.chunk_count {
            self.file_repo.abort_transfer(session.session_id)?;
            return Err(anyhow!(
                "Upload incomplete: expected {} chunks, got {}",
                session.chunk_count,
                session.chunks_uploaded
            ));
        }

        self.file_repo.end_transfer(session.session_id)?;

        let mut updated_revision = session.revision;
        updated_revision.upload_ended_on = Some(Utc::now().naive_utc());
        self.revision_repo.update_revision(updated_revision)?;

        let mut updated_node = session.node;
        updated_node.current_revision = Some(updated_revision.id);
        self.node_repo.update_node(updated_node)?;

        Ok(())
    }

    pub fn replace_file(
        &self,
        node_id: NodeId,
        node_metadata: EncryptedMetadata,
        chunk_count: ChunkIndex,
        iv: IV,
    ) -> Result<NodeId> {
        let existing_node = self.node_repo.get_node(node_id)?;

        if !matches!(existing_node.node_type, NodeType::File) {
            return Err(anyhow!("Node {} is not a file", node_id));
        }

        let new_revision = self.revision_repo.create_revision(
            node_id,
            Utc::now().naive_utc(),
            iv,
        )?;

        let session_id = self.file_repo.start_transfer(node_id.to_string())?;

        let mut updated_node = existing_node;
        updated_node.metadata = node_metadata;
        updated_node.metadata_change_counter += 1;

        let mut temp = self.temp_storage.lock().unwrap();
        temp.nodes.insert(
            node_id,
            UploadSession {
                node: updated_node,
                revision: new_revision,
                chunk_count,
                session_id,
                chunks_uploaded: 0,
            },
        );

        Ok(node_id)
    }

    pub fn finish_replacement(&self, node_id: NodeId) -> Result<()> {
        let mut temp = self.temp_storage.lock().unwrap();
        let session = temp
            .nodes
            .remove(&node_id)
            .ok_or_else(|| anyhow!("Node {} not found in temporary storage", node_id))?;
        drop(temp);

        if session.chunks_uploaded != session.chunk_count {
            self.file_repo.abort_transfer(session.session_id)?;
            return Err(anyhow!(
                "Upload incomplete: expected {} chunks, got {}",
                session.chunk_count,
                session.chunks_uploaded
            ));
        }

        self.file_repo.end_transfer(session.session_id)?;

        let mut updated_revision = session.revision;
        updated_revision.upload_ended_on = Some(Utc::now().naive_utc());
        self.revision_repo.update_revision(updated_revision)?;

        let mut updated_node = session.node;
        updated_node.current_revision = Some(updated_revision.id);
        self.node_repo.update_node(updated_node)?;

        Ok(())
    }

    pub fn abort_upload(&self, node_id: NodeId) -> Result<()> {
        let mut temp = self.temp_storage.lock().unwrap();
        if let Some(session) = temp.nodes.remove(&node_id) {
            drop(temp);
            self.file_repo.abort_transfer(session.session_id)?;
            self.revision_repo.delete_revision(session.revision.id)?;
            self.node_repo.purge_tree(node_id)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO: Tests implementieren mit Mock-Repositories
}