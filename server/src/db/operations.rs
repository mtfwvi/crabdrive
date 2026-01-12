use crate::storage::file::file_repository::FileRepository;
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
    nodes: HashMap<NodeId, (NodeEntity, RevisionEntity, ChunkIndex)>,
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

        self.node_repo.update_node(updated_parent)?;

        let node =
            self.node_repo
                .create_node(Some(parent_id), node_metadata, owner, NodeType::File)?;

        let node_id = node.id;

        let revision = self
            .revision_repo
            .create_revision(node_id, Utc::now().naive_utc(), iv)?;

        let session_id = self.file_repo.start_transfer(node_id.to_string())?;

        let mut temp = self.temp_storage.lock().unwrap();
        temp.nodes.insert(node_id, (node, revision, chunk_count));

        Ok(node_id)
    }

    pub fn upload_chunk(
        &self,
        node_id: NodeId,
        chunk: crate::storage::file::model::FileChunk,
    ) -> Result<()> {
        let temp = self.temp_storage.lock().unwrap();
        if !temp.nodes.contains_key(&node_id) {
            return Err(anyhow!("Node {} not found in temporary storage", node_id));
        }
        drop(temp);

        let file_key = node_id.to_string();
        Ok(())
    }

    pub fn finish_upload(&self, node_id: NodeId) -> Result<()> {
        let mut temp = self.temp_storage.lock().unwrap();
        let (node, mut revision, _chunk_count) = temp
            .nodes
            .remove(&node_id)
            .ok_or_else(|| anyhow!("Node {} not found in temporary storage", node_id))?;
        drop(temp);

        let revision_entity = self.revision_repo.get_revision(revision.id)?;
        let mut updated_revision = revision_entity;

        self.revision_repo.update_revision(updated_revision)?;

        Ok(())
    }

    pub fn replace_file(
        &self,
        node_id: NodeId,
        _node_metadata: EncryptedMetadata,
        iv: IV,
    ) -> Result<NodeId> {
        let existing_node = self.node_repo.get_node(node_id)?;

        if !matches!(existing_node.node_type, NodeType::File) {
            return Err(anyhow!("Node {} is not a file", node_id));
        }

        let new_revision =
            self.revision_repo
                .create_revision(node_id, Utc::now().naive_utc(), iv)?;

        let _session_id = self.file_repo.start_transfer(node_id.to_string())?;

        let temp_node = existing_node;

        let mut temp = self.temp_storage.lock().unwrap();
        temp.nodes.insert(node_id, (temp_node, new_revision, 0));

        Ok(node_id)
    }

    pub fn finish_replacement(&self, node_id: NodeId) -> Result<()> {
        let mut temp = self.temp_storage.lock().unwrap();
        let (temp_node,revision, _) = temp
            .nodes
            .remove(&node_id)
            .ok_or_else(|| anyhow!("Node {} not found in temporary storage", node_id))?;
        drop(temp);

        let revision_entity = self.revision_repo.get_revision(revision.id)?;
        self.revision_repo.update_revision(revision_entity)?;
        self.node_repo.update_node(temp_node)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
