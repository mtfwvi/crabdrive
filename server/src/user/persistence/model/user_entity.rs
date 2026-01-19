use crate::user::persistence::model::encryption_key::EncryptionKey;
use chrono::NaiveDateTime;
use crabdrive_common::data::DataAmount;
use crabdrive_common::storage::NodeId;
use crabdrive_common::user::{UserId, UserType};

pub(crate) struct UserEntity {
    user_type: UserType,
    created_at: NaiveDateTime,
    id: UserId,

    username: String,
    password_hash: String,
    storage_limit: DataAmount,
    encryption_uninitialized: bool, // default: false

    // encrypted with key derived from user password
    master_key: EncryptionKey,

    // encrypted with master key
    private_key: EncryptionKey,

    // not encrypted (needs to be verified before each usage as the server could modify it
    public_key: Vec<u8>,

    // encrypted with master key
    // used to encrypt the users root folder metadata
    root_key: EncryptionKey,

    // should be created when the user first logs in
    root_node: Option<NodeId>,

    // encrypted with master key
    // used to encrypt the trash folder metadata
    trash_key: EncryptionKey,

    // should be created when the user first logs in
    trash_node: Option<NodeId>,
}
