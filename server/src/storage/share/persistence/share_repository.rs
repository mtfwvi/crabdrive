use crate::db::connection::DbPool;
use crate::db::operations::{
    delete_share, get_all_shares_by_node, get_all_shares_by_user, get_share_by_user_node,
    insert_share, select_share, update_share,
};
use crate::storage::share::persistence::model::share_entity::ShareEntity;
use anyhow::Result;
use crabdrive_common::storage::{NodeId, ShareId};
use crabdrive_common::user::UserId;
use std::sync::Arc;

pub trait ShareRepository {
    fn get_share(&self, share_id: ShareId) -> Result<Option<ShareEntity>>;
    fn insert_share(&self, entity: ShareEntity) -> Result<ShareEntity>;
    fn delete_share(&self, share_id: ShareId) -> Result<ShareEntity>;
    fn update_share(&self, entity: ShareEntity) -> Result<ShareEntity>;
    fn get_accepted_shares_by_user(&self, user_id: UserId) -> Result<Vec<ShareEntity>>;
    fn get_shares_by_node_id(&self, node_id: NodeId) -> Result<Vec<ShareEntity>>;
    fn get_share_by_node_id_and_user_id(
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
        select_share(&self.db_pool, share_id)
    }

    fn insert_share(&self, entity: ShareEntity) -> Result<ShareEntity> {
        insert_share(&self.db_pool, &entity)
    }

    fn delete_share(&self, share_id: ShareId) -> Result<ShareEntity> {
        delete_share(&self.db_pool, share_id)
    }

    fn update_share(&self, entity: ShareEntity) -> Result<ShareEntity> {
        update_share(&self.db_pool, &entity)
    }

    fn get_accepted_shares_by_user(&self, user_id: UserId) -> Result<Vec<ShareEntity>> {
        get_all_shares_by_user(&self.db_pool, user_id)
    }

    fn get_shares_by_node_id(&self, node_id: NodeId) -> Result<Vec<ShareEntity>> {
        get_all_shares_by_node(&self.db_pool, node_id)
    }

    fn get_share_by_node_id_and_user_id(
        &self,
        node_id: NodeId,
        user_id: UserId,
    ) -> Result<Option<ShareEntity>> {
        get_share_by_user_node(&self.db_pool, node_id, user_id)
    }
}
