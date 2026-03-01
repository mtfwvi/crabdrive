use crate::user::UserEntity;
use chrono::NaiveDateTime;
use crabdrive_common::user::UserId;
use crabdrive_common::uuid::UUID;
use diesel::prelude::Associations;

use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

pub type SessionId = UUID;

#[derive(
    Associations,
    Queryable,
    Selectable,
    Serialize,
    Deserialize,
    Debug,
    Insertable,
    AsChangeset,
    Clone,
)]
#[diesel(table_name = crate::db::schema::RefreshToken)]
#[diesel(belongs_to(UserEntity, foreign_key = user_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RefreshTokenEntity {
    /// The hash of the token
    pub token: Vec<u8>,
    /// The ID of the user assosciated with the token
    pub user_id: UserId,
    /// All tokens issued in one session have the same family. Allows for deleting old tokens when logging out.
    pub session_id: SessionId,
    /// Datetime when the
    pub expires_at: NaiveDateTime,
    /// If a token is refreshed, `invalidated_at` will be set to the current timestamp.
    pub invalidated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Insertable, AsChangeset, Clone)]
#[diesel(table_name = crate::db::schema::TokenBlacklist)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BlacklistedTokenEntity {
    /// The JTI of the token
    pub id: String,
    /// Date when the tokes
    pub expires_at: NaiveDateTime,
}
