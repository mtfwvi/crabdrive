// This code was largely sourced from Samuel Rodrigues (Obito1903).
// Licensed under CC BY-NC 4.0 (https://creativecommons.org/licenses/by-nc/4.0/)
// https://obito.fr/posts/2022/12/use-uuid-in-sqlite-database-with-rust-diesel.rs/

use std::fmt;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    serialize::{self, IsNull, Output, ToSql},
    sql_types::Text,
    sqlite::Sqlite,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "server", diesel(sql_type = Text))]
pub struct UUID(uuid::Uuid);

// Small function to easily initialize our uuid
impl UUID {
    pub fn random() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn nil() -> Self {
        Self(uuid::Uuid::nil())
    }

    pub fn parse_string(s: String) -> Option<UUID> {
        match uuid::Uuid::parse_str(&s) {
            Ok(uuid) => Some(UUID(uuid)),
            Err(_) => None,
        }
    }
}

// Allow easy conversion from uuid::Uuid to UUID
impl From<uuid::Uuid> for UUID {
    fn from(s: uuid::Uuid) -> Self {
        Self(s)
    }
}

// Allow easy conversion from UUID to the wanted uuid::Uuid
impl From<UUID> for uuid::Uuid {
    fn from(s: UUID) -> Self {
        s.0
    }
}

impl Display for UUID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "server")]
impl FromSql<Text, Sqlite> for UUID {
    fn from_sql(
        bytes: <Sqlite as diesel::backend::Backend>::RawValue<'_>,
    ) -> deserialize::Result<Self> {
        let text = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        let uuid = uuid::Uuid::parse_str(&text)?;
        Ok(Self(uuid))
    }
}

#[cfg(feature = "server")]
impl ToSql<Text, Sqlite> for UUID {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(format!("{}", self.0));
        Ok(IsNull::No)
    }
}
