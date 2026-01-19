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
