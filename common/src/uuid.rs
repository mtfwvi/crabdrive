use diesel::deserialize::FromSqlRow;
use diesel::deserialize::{self, FromSql};
use diesel::expression::AsExpression;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};
use uuid::Uuid;
use diesel::deserialize::{self, FromSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;


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

use diesel::deserialize::{self, FromSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use uuid::Uuid;

impl FromSql<Text, Sqlite> for UUID {
    fn from_sql(bytes: diesel::backend::RawValue<Sqlite>) -> deserialize::Result<Self> {
        // Für Sqlite ist RawValue ein &[u8], wir müssen es als String interpretieren
        let text = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        let uuid = Uuid::parse_str(&text)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        Ok(Self(uuid))
    }
}

impl ToSql<Text, Sqlite> for UUID {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(format!("{}", self.0));
        Ok(IsNull::No)
    }
}
