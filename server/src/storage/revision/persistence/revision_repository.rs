use std::sync::Arc;

use crate::db;
use crate::db::connection::DbPool;
use crate::storage::revision::persistence::model::revision_entity::RevisionEntity;
use anyhow::Result;
use chrono::NaiveDateTime;
use crabdrive_common::iv::IV;
use crabdrive_common::storage::ChunkIndex;
use crabdrive_common::storage::NodeId;
use crabdrive_common::storage::RevisionId;
use crabdrive_common::uuid::UUID;

pub(crate) trait RevisionRepository {
    fn get_revision(&self, revision_id: RevisionId) -> Result<Option<RevisionEntity>>;
    fn get_all_revisions_by_node(&self, node_id: NodeId) -> Result<Vec<RevisionEntity>>;
    fn create_revision(
        &self,
        node_id: NodeId,
        upload_started_on: NaiveDateTime,
        iv: IV,
        chunk_count: ChunkIndex,
    ) -> Result<RevisionEntity>;
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
            id: UUID::random(),
            file_id,
            upload_started_on,
            upload_ended_on: None,
            iv,
            chunk_count,
        };
        db::operations::insert_revision(&self.db_pool, &revision)
    }

    fn update_revision(&self, file_version: RevisionEntity) -> Result<()> {
        db::operations::update_revision(&self.db_pool, &file_version)?;
        Ok(())
    }

    fn get_revision(&self, id: RevisionId) -> Result<Option<RevisionEntity>> {
        db::operations::select_revision(&self.db_pool, id)
    }

    fn delete_revision(&self, id: RevisionId) -> Result<RevisionEntity> {
        db::operations::delete_revision(&self.db_pool, id)
    }

    fn get_all_revisions_by_node(&self, node_id: NodeId) -> Result<Vec<RevisionEntity>> {
        db::operations::get_all_revisions_by_node(&self.db_pool, node_id)
    }

    fn get_revision_history(&self, node_id: NodeId) -> Result<Vec<RevisionEntity>> {
        let mut revisions = db::operations::get_all_revisions_by_node(&self.db_pool, node_id)?;
        revisions.retain(|r| r.upload_ended_on.is_some());
        revisions.sort_by(|a, b| b.upload_ended_on.unwrap().cmp(&a.upload_ended_on.unwrap()));
        Ok(revisions)
    }
}
