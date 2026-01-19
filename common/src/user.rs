use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};

use crate::uuid::UUID;

pub type UserId = UUID;

#[derive(Debug, Serialize, Deserialize, FromSqlRow, PartialEq, AsExpression, Clone)]
#[diesel(sql_type = Text)]
pub enum UserType {
    User,
    Admin,
    Restricted,
}

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
