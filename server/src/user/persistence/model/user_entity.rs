use chrono::NaiveDateTime;
use crabdrive_common::data::DataAmount;
use crabdrive_common::encryption_key::EncryptionKey;
use crabdrive_common::storage::NodeId;
use crabdrive_common::user::{UserId, UserType};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Insertable, AsChangeset, Clone)]
#[diesel(table_name = crate::db::schema::User)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(encryptionKey))]
pub(crate) struct UserEntity {
    pub user_type: UserType,
    pub id: UserId,
    pub created_at: NaiveDateTime,

    pub username: String,
    pub password_hash: String,
    pub storage_limit: DataAmount,
    pub storage_used: DataAmount,
    pub encryption_uninitialized: bool, // default: false

    // encrypted with key derived from user password
    pub master_key: EncryptionKey,

    // encrypted with master key
    pub private_key: EncryptionKey,

    // not encrypted (needs to be verified before each usage as the server could modify it
    pub public_key: Vec<u8>,

    // encrypted with master key
    // used to encrypt the users root folder metadata
    pub root_key: EncryptionKey,

    // should be created when the user first logs in
    pub root_node: Option<NodeId>,

    // encrypted with master key
    // used to encrypt the trash folder metadata
    pub trash_key: EncryptionKey,

    // should be created when the user first logs in
    pub trash_node: Option<NodeId>,
}
