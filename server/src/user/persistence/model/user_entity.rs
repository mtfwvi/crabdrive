use crate::user::persistence::model::encryption_key::EncryptionKey;
use chrono::NaiveDateTime;
use crabdrive_common::data::DataAmount;
use crabdrive_common::storage::NodeId;
use crabdrive_common::user::UserId;
use rusqlite::Result;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};

pub(crate) enum UserType {
    User,
    Admin,
    Restricted,
}

impl ToSql for UserType {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let s = match self {
            UserType::User => "user",
            UserType::Admin => "admin",
            UserType::Restricted => "restricted",
        };
        Ok(ToSqlOutput::from(s))
    }
}

impl FromSql for UserType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Text(s) => {
                let s = std::str::from_utf8(s).map_err(|e| FromSqlError::Other(Box::new(e)))?;
                match s {
                    "user" => Ok(UserType::User),
                    "admin" => Ok(UserType::Admin),
                    "restricted" => Ok(UserType::Restricted),
                    _ => Err(FromSqlError::Other(
                        format!("Invalid UserType: {}", s).into(),
                    )),
                }
            }
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

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
