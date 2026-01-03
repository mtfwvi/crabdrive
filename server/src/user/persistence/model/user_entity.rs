use crate::user::persistence::model::encryption_key::EncryptionKey;
use chrono::NaiveDateTime;
use crabdrive_common::data::DataAmount;
use crabdrive_common::storage::NodeId;
use crabdrive_common::user::UserId;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;

#[derive(Debug)]
pub(crate) enum UserType {
    User,
    Admin,
    Restricted,
}

impl ToSql<Text, Sqlite> for UserType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let value = match self {
            UserType::User => "user",
            UserType::Admin => "admin",
            UserType::Restricted => "restricted",
        };

        out.set_value(value);
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Sqlite> for UserType {
    fn from_sql(
        bytes: <Sqlite as diesel::backend::Backend>::RawValue<'_>,
    ) -> deserialize::Result<Self> {
        let s = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;

        match s.as_str() {
            "user" => Ok(UserType::User),
            "admin" => Ok(UserType::Admin),
            "restricted" => Ok(UserType::Restricted),
            _ => Err(format!("Invalid UserType: {}", s).into()),
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
