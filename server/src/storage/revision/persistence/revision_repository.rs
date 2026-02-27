use crate::db::connection::DbPool;
use crate::db::operations::revision::*;
use crate::storage::revision::RevisionEntity;

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

    /// Delete a revision
    fn delete_revision(&self, revision_id: RevisionId) -> Result<RevisionEntity>;
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
        let mut conn = self.db_pool.get()?;

        let revision = RevisionEntity {
            id: RevisionId::random(),
            file_id,
            upload_started_on,
            upload_ended_on: None,
            iv,
            chunk_count,
        };

        insert_revision(&mut conn, &revision)
    }

    fn update_revision(&self, file_version: RevisionEntity) -> Result<()> {
        let mut conn = self.db_pool.get()?;
        update_revision(&mut conn, &file_version)?;
        Ok(())
    }

    fn get_revision(&self, id: RevisionId) -> Result<Option<RevisionEntity>> {
        let mut conn = self.db_pool.get()?;
        select_revision(&mut conn, id)
    }

    fn delete_revision(&self, id: RevisionId) -> Result<RevisionEntity> {
        let mut conn = self.db_pool.get()?;
        delete_revision(&mut conn, id)
    }

    fn get_all_revisions_by_node(&self, node_id: NodeId) -> Result<Vec<RevisionEntity>> {
        let mut conn = self.db_pool.get()?;
        get_all_revisions_by_node(&mut conn, node_id)
    }
}
