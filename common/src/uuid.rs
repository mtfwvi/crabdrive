// This code was largely sourced from Samuel Rodrigues (Obito1903).
// Licensed under CC BY-NC 4.0 (https://creativecommons.org/licenses/by-nc/4.0/)
// https://obito.fr/posts/2022/12/use-uuid-in-sqlite-database-with-rust-diesel.rs/

use diesel::deserialize::FromSqlRow;
use diesel::deserialize::{self, FromSql};
use diesel::expression::AsExpression;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(
    Debug, Clone, Copy, FromSqlRow, Hash, Eq, PartialEq, Serialize, Deserialize, AsExpression,
)]
#[diesel(sql_type = Text)]
pub struct UUID(pub uuid::Uuid);

// Small function to easily initialize our uuid
impl UUID {
    pub fn random() -> Self {
        Self(uuid::Uuid::new_v4())
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

impl FromSql<Text, Sqlite> for UUID {
    fn from_sql(
        bytes: <Sqlite as diesel::backend::Backend>::RawValue<'_>,
    ) -> deserialize::Result<Self> {
        // Für Sqlite ist RawValue ein &[u8], wir müssen es als String interpretieren
        let text = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        let uuid = uuid::Uuid::parse_str(&text)?;
        Ok(Self(uuid))
    }
}

impl ToSql<Text, Sqlite> for UUID {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(format!("{}", self.0));
        Ok(IsNull::No)
    }
}
