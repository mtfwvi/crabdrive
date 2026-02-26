use crate::encryption_key::EncryptionKey;
use crate::uuid::UUID;

use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    serialize::{self, IsNull, Output, ToSql},
    sql_types::Text,
    sqlite::Sqlite,
};

pub type UserId = UUID;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "server", diesel(sql_type = Text))]
pub enum UserType {
    User,
    Admin,
    Restricted,
}

#[cfg(feature = "server")]
impl ToSql<Text, Sqlite> for UserType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let value = match self {
            UserType::User => "USER",
            UserType::Admin => "ADMIN",
            UserType::Restricted => "RESTRICTED",
        };

        out.set_value(value);
        Ok(IsNull::No)
    }
}

#[cfg(feature = "server")]
impl FromSql<Text, Sqlite> for UserType {
    fn from_sql(
        bytes: <Sqlite as diesel::backend::Backend>::RawValue<'_>,
    ) -> deserialize::Result<Self> {
        let s = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;

        match s.as_str() {
            "USER" => Ok(UserType::User),
            "ADMIN" => Ok(UserType::Admin),
            "RESTRICTED" => Ok(UserType::Restricted),
            _ => Err(format!("Invalid UserType: {}", s).into()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserKeys {
    pub public_key: Vec<u8>,
    pub private_key: EncryptionKey,
    pub master_key: EncryptionKey,
    pub root_key: EncryptionKey,
    pub trash_key: EncryptionKey,
}

impl UserKeys {
    pub fn new(
        public_key: Vec<u8>,
        private_key: EncryptionKey,
        master_key: EncryptionKey,
        root_key: EncryptionKey,
        trash_key: EncryptionKey,
    ) -> Self {
        Self {
            public_key,
            private_key,
            master_key,
            root_key,
            trash_key,
        }
    }

    pub fn nil() -> Self {
        Self {
            public_key: vec![],
            private_key: EncryptionKey::nil(),
            master_key: EncryptionKey::nil(),
            root_key: EncryptionKey::nil(),
            trash_key: EncryptionKey::nil(),
        }
    }

    #[cfg(any(test, feature = "server-tests"))]
    pub fn random() -> Self {
        use rand::RngCore;

        let mut public_key = vec![0u8; 32];
        rand::rng().fill_bytes(&mut public_key);

        Self {
            public_key,
            private_key: EncryptionKey::random(),
            master_key: EncryptionKey::random(),
            root_key: EncryptionKey::random(),
            trash_key: EncryptionKey::random(),
        }
    }
}
