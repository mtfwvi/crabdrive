use crate::db::connection::DbPool;
use crate::db::operations::share::*;
use crate::storage::share::ShareEntity;

use crabdrive_common::encryption_key::EncryptionKey;
use crabdrive_common::storage::{NodeId, ShareId};
use crabdrive_common::user::UserId;

use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;

pub trait ShareRepository {
    /// Get a share entry by ID
    fn get_share(&self, share_id: ShareId) -> Result<Option<ShareEntity>>;
    /// Create a new share entry
    fn create_share(
        &self,
        node_id: NodeId,
        shared_by: UserId,
        key: EncryptionKey,
    ) -> Result<ShareEntity>;
    /// Delete a share entry
    fn delete_share(&self, share_id: ShareId) -> Result<ShareEntity>;
    /// Update a share entry
    fn update_share(&self, entity: ShareEntity) -> Result<ShareEntity>;
    /// Get all share entries by User ID
    fn get_accepted_shares_by_user(&self, user_id: UserId) -> Result<Vec<ShareEntity>>;
    /// Get all share entries for a Node
    fn get_shares_by_node_id(&self, node_id: NodeId) -> Result<Vec<ShareEntity>>;
    /// Get a share entry by the Node ID and the User ID which accepted the share
    fn get_share_by_node_id_and_accepted_user_id(
        &self,
        node_id: NodeId,
        user_id: UserId,
    ) -> Result<Option<ShareEntity>>;
}

pub struct ShareRepositoryImpl {
    db_pool: Arc<DbPool>,
}

impl ShareRepositoryImpl {
    pub fn new(db_pool: Arc<DbPool>) -> Self {
        Self { db_pool }
    }
}

impl ShareRepository for ShareRepositoryImpl {
    fn get_share(&self, share_id: ShareId) -> Result<Option<ShareEntity>> {
        let mut conn = self.db_pool.get()?;
        select_share(&mut conn, share_id)
    }

    fn create_share(
        &self,
        node_id: NodeId,
        shared_by: UserId,
        key: EncryptionKey,
    ) -> Result<ShareEntity> {
        let mut conn = self.db_pool.get()?;

        let share_entity = ShareEntity {
            id: ShareId::random(),
            node_id,
            shared_by,
            accepted_by: None,
            time_shared: Utc::now().naive_utc(),
            time_accepted: None,
            shared_encryption_key: Some(key),
            accepted_encryption_key: None,
        };

        insert_share(&mut conn, &share_entity)
    }

    fn delete_share(&self, share_id: ShareId) -> Result<ShareEntity> {
        let mut conn = self.db_pool.get()?;
        delete_share(&mut conn, share_id)
    }

    fn update_share(&self, entity: ShareEntity) -> Result<ShareEntity> {
        let mut conn = self.db_pool.get()?;
        update_share(&mut conn, &entity)
    }

    fn get_accepted_shares_by_user(&self, user_id: UserId) -> Result<Vec<ShareEntity>> {
        let mut conn = self.db_pool.get()?;
        get_all_shares_by_user(&mut conn, user_id)
    }

    fn get_shares_by_node_id(&self, node_id: NodeId) -> Result<Vec<ShareEntity>> {
        let mut conn = self.db_pool.get()?;
        get_all_shares_by_node(&mut conn, node_id)
    }

    fn get_share_by_node_id_and_accepted_user_id(
        &self,
        node_id: NodeId,
        user_id: UserId,
    ) -> Result<Option<ShareEntity>> {
        let mut conn = self.db_pool.get()?;
        get_share_by_node_id_and_accepted_user_id(&mut conn, node_id, user_id)
    }
}
