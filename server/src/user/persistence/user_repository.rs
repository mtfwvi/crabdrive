use crate::db::connection::DbPool;
use crate::db::operations::user::*;
use crate::user::UserEntity;

use crabdrive_common::data::DataAmount;
use crabdrive_common::user::{UserId, UserKeys, UserType};

use std::sync::Arc;

use anyhow::{Context, Result};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher};
use argon2::{PasswordHash, PasswordVerifier};
use chrono::Utc;

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
    fn get_user(&self, id: UserId) -> Result<Option<UserEntity>>;
    /// Validate the password hash of a user
    fn authenticate_user(&self, username: &str, password: &str) -> Result<Option<UserEntity>>;
    /// Get a user by their username
    fn get_user_by_username(&self, username: &str) -> Result<Option<UserEntity>>;
    /// Update a username
    fn update_user(&self, updated_entity: UserEntity) -> Result<UserEntity>;
    /// Hard-delete a user from the database
    fn delete_user(&self, id: UserId) -> Result<UserEntity>;
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
        username: &str,
        password: &str,
        storage_limit: DataAmount,
        keys: UserKeys,
    ) -> Result<UserEntity> {
        let mut conn = self.db_pool.get()?;

        let password_salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &password_salt)
            .unwrap()
            .to_string();

        let user = UserEntity {
            user_type: UserType::User,
            id: UserId::random(),
            created_at: Utc::now().naive_utc(),
            username: username.to_string(),
            password_hash,
            storage_limit,
            // Currently unused. Maybe useful for admin routes.
            encryption_uninitialized: false,
            master_key: keys.master_key,
            private_key: keys.private_key,
            public_key: keys.public_key,
            root_key: keys.root_key,

            root_node: None,
            trash_key: keys.trash_key,
            trash_node: None,
        };

        insert_user(&mut conn, &user).context("Failed to insert user")?;
        Ok(user)
    }

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

    fn get_user(&self, id: UserId) -> Result<Option<UserEntity>> {
        let mut conn = self.db_pool.get()?;
        select_user(&mut conn, id).context("Failed to select user")
    }

    fn get_user_by_username(&self, username: &str) -> Result<Option<UserEntity>> {
        let mut conn = self.db_pool.get()?;
        select_user_by_username(&mut conn, username).context("Failed to select user by username")
    }

    fn update_user(&self, updated_entity: UserEntity) -> Result<UserEntity> {
        let mut conn = self.db_pool.get()?;
        update_user(&mut conn, &updated_entity).context("Failed to update user")
    }

    fn delete_user(&self, id: UserId) -> Result<UserEntity> {
        let mut conn = self.db_pool.get()?;
        delete_user(&mut conn, id).context("Failed to delete user")
    }
}
