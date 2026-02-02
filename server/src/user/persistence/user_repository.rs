use crate::db::connection::DbPool;
use crate::db::operations::{delete_user, insert_user, select_user, update_user};
use crate::user::persistence::model::encryption_key::EncryptionKey;
use crate::user::persistence::model::user_entity::UserEntity;
use anyhow::Context;
use anyhow::Result;
use chrono::Utc;
use crabdrive_common::user::UserType;
use crabdrive_common::{data::DataAmount, uuid::UUID};
use std::sync::Arc;

pub(crate) trait UserRepository {
    fn create_user(
        &self,
        username: String,
        password_hash: String,
        storage_limit: DataAmount,
    ) -> Result<UserEntity>;
    fn get_user(&self, id: UUID) -> Result<Option<UserEntity>>;
    fn update_user(&self, updated_entity: UserEntity) -> Result<()>;
    fn delete_user(&self, id: UUID) -> Result<UserEntity>;
}

pub struct UserState {
    db_pool: Arc<DbPool>,
}

impl UserState {
    pub fn new(db_pool: Arc<DbPool>) -> Self {
        Self { db_pool }
    }
}

impl UserRepository for UserState {
    fn create_user(
        &self,
        username: String,
        password_hash: String,
        storage_limit: DataAmount,
    ) -> Result<UserEntity> {
        let user = UserEntity {
            user_type: UserType::User,
            id: UUID::random(),
            created_at: Utc::now().naive_utc(),
            username,
            password_hash,
            storage_limit,
            encryption_uninitialized: true,
            master_key: EncryptionKey::nil(),
            private_key: EncryptionKey::nil(),
            public_key: Vec::new(),
            root_key: EncryptionKey::nil(),
            root_node: None,
            trash_key: EncryptionKey::nil(),
            trash_node: None,
        };
        insert_user(&self.db_pool, &user).context("Failed to insert user")?;
        Ok(user)
    }

    fn get_user(&self, id: UUID) -> Result<Option<UserEntity>> {
        select_user(&self.db_pool, id).context("Failed to select user")
    }

    fn update_user(&self, updated_entity: UserEntity) -> Result<()> {
        update_user(&self.db_pool, &updated_entity).context("Failed to update user")
    }

    fn delete_user(&self, id: UUID) -> Result<UserEntity> {
        delete_user(&self.db_pool, id).context("Failed to delete user")
    }
}
