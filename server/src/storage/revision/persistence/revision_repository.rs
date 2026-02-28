use crate::db::connection::DbPool;
use crate::db::operations::{
    delete_revision, get_all_revisions_by_node, insert_revision, select_revision, update_revision,
};
use crate::storage::revision::persistence::model::revision_entity::RevisionEntity;
use crabdrive_common::iv::IV;
use crabdrive_common::storage::{ChunkIndex, NodeId, RevisionId};
use std::sync::Arc;
use anyhow::Result;
use chrono::NaiveDateTime;

pub(crate) trait RevisionRepository {
    /// Creates a new (unfinished) revision, associated with a node.
    fn create_revision(
        &self,
        node_id: NodeId,
        upload_started_on: NaiveDateTime,
        iv: IV,
        chunk_count: ChunkIndex,
    ) -> Result<RevisionEntity>;

    /// Query a revision by its ID
    fn get_revision(&self, revision_id: RevisionId) -> Result<Option<RevisionEntity>>;

    /// Query all revisions associated with a node.
    fn get_all_revisions_by_node(&self, node_id: NodeId) -> Result<Vec<RevisionEntity>>;

    /// Patches an existing revision
    fn update_revision(&self, revision_entity: RevisionEntity) -> Result<()>;

    fn delete_revision(&self, revision_id: RevisionId) -> Result<RevisionEntity>;

    fn get_revision_history(&self, node_id: NodeId) -> Result<Vec<RevisionEntity>>;
}

pub struct RevisionService {
    db_pool: Arc<DbPool>,
}

impl RevisionService {
    pub fn new(db_pool: Arc<DbPool>) -> Self {
        Self { db_pool }
    }
}

impl RevisionRepository for RevisionService {
    fn create_revision(
        &self,
        file_id: NodeId,
        upload_started_on: NaiveDateTime,
        iv: IV,
        chunk_count: ChunkIndex,
    ) -> Result<RevisionEntity> {
        let revision = RevisionEntity {
            id: RevisionId::random(),
            file_id,
            upload_started_on,
            upload_ended_on: None,
            iv,
            chunk_count,
        };
        insert_revision(&self.db_pool, &revision)
    }

    fn get_revision(&self, id: RevisionId) -> Result<Option<RevisionEntity>> {
        select_revision(&self.db_pool, id)
    }

    fn get_all_revisions_by_node(&self, node_id: NodeId) -> Result<Vec<RevisionEntity>> {
        get_all_revisions_by_node(&self.db_pool, node_id)
    }

    fn update_revision(&self, file_version: RevisionEntity) -> Result<()> {
        update_revision(&self.db_pool, &file_version)?;
        Ok(())
    }

    fn delete_revision(&self, id: RevisionId) -> Result<RevisionEntity> {
        delete_revision(&self.db_pool, id)
    }

    fn get_revision_history(&self, node_id: NodeId) -> Result<Vec<RevisionEntity>> {
        let mut revisions = self.get_all_revisions_by_node(node_id)?;
        revisions.retain(|r| r.upload_ended_on.is_some());
        revisions.sort_by(|a, b| b.upload_ended_on.unwrap().cmp(&a.upload_ended_on.unwrap()));
        Ok(revisions)
    }
}