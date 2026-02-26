use crate::db::connection::DbPool;
use crate::db::operations::{
    delete_user, insert_user, select_user, select_user_by_username, update_user,
};
use crate::user::persistence::model::user_entity::UserEntity;
use anyhow::Context;
use anyhow::Result;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher};
use argon2::{PasswordHash, PasswordVerifier};
use chrono::Utc;
use crabdrive_common::user::{UserKeys, UserType};
use crabdrive_common::{da, data::DataAmount, uuid::UUID};
use std::sync::Arc;
use tracing::instrument;

pub(crate) trait UserRepository {
    /// Create a new user
    fn create_user(
        &self,
        username: &str,
        password: &str,
        storage_limit: DataAmount,
        keys: UserKeys,
    ) -> Result<UserEntity>;
    /// Get a user by ID
    fn get_user(&self, id: UUID) -> Result<Option<UserEntity>>;
    /// Validate the password hash of a user
    fn authenticate_user(&self, username: &str, password: &str) -> Result<Option<UserEntity>>;
    /// Get a user by their username
    fn get_user_by_username(&self, username: &str) -> Result<Option<UserEntity>>;
    /// Update a username
    fn update_user(&self, updated_entity: UserEntity) -> Result<UserEntity>;
    /// Hard-delete a user from the database
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
    #[instrument(skip(self, password), err)]
    fn create_user(
        &self,
        username: &str,
        password: &str,
        storage_limit: DataAmount,
        keys: UserKeys,
    ) -> Result<UserEntity> {
        let password_salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &password_salt)
            .unwrap()
            .to_string();

        let user = UserEntity {
            user_type: UserType::User,
            id: UUID::random(),
            created_at: Utc::now().naive_utc(),
            username: username.to_string(),
            password_hash: password_hash.to_string(),
            storage_limit,
            // Currently unused. Maybe useful for admin routes.
            storage_used: da!(0 B),
            encryption_uninitialized: false,
            master_key: keys.master_key,
            private_key: keys.private_key,
            public_key: keys.public_key,
            root_key: keys.root_key,

            root_node: None,
            trash_key: keys.trash_key,
            trash_node: None,
        };

        insert_user(&self.db_pool, &user).context("Failed to insert user")?;
        Ok(user)
    }

    #[instrument(skip(self, password), err)]
    fn authenticate_user(&self, username: &str, password: &str) -> Result<Option<UserEntity>> {
        let Some(user) = self.get_user_by_username(username)? else {
            tracing::debug!("User not found");
            return Ok(None);
        };

        let parsed_hash = PasswordHash::new(&user.password_hash).expect("Failed to hash password!");
        if Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_err()
        {
            tracing::debug!("Wrong password!");
            return Ok(None);
        }

        Ok(Some(user))
    }

    #[instrument(skip(self), err)]
    fn get_user(&self, id: UUID) -> Result<Option<UserEntity>> {
        select_user(&self.db_pool, id).context("Failed to select user")
    }

    #[instrument(skip(self), err)]
    fn get_user_by_username(&self, username: &str) -> Result<Option<UserEntity>> {
        select_user_by_username(&self.db_pool, username)
            .context("Failed to select user by username")
    }

    #[instrument(skip(self), err)]
    fn update_user(&self, updated_entity: UserEntity) -> Result<UserEntity> {
        update_user(&self.db_pool, &updated_entity).context("Failed to update user")
    }

    #[instrument(skip(self), err)]
    fn delete_user(&self, id: UUID) -> Result<UserEntity> {
        delete_user(&self.db_pool, id).context("Failed to delete user")
    }
}
