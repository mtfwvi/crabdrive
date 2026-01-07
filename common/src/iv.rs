use diesel::backend::Backend;
use diesel::deserialize::FromSqlRow;
use diesel::deserialize::{self, FromSql};
use diesel::expression::AsExpression;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Binary;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[derive(Debug, Clone, Copy, FromSqlRow, Serialize, Deserialize, PartialEq, AsExpression)]
#[diesel(sql_type = Binary)]
pub struct IV([u8; 12]);

impl IV {
    pub fn new(iv: [u8; 12]) -> Self {
        IV(iv)
    }

    pub fn get(&self) -> [u8; 12] {
        self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl ToSql<Binary, Sqlite> for IV {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(self.0.as_slice());
        Ok(IsNull::No)
    }
}

impl FromSql<Binary, Sqlite> for IV {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let bytes_vec = Vec::<u8>::from_sql(bytes)?;

        let array: [u8; 12] = bytes_vec.try_into().map_err(|_| "IV not 12 bytes")?;

        Ok(IV(array))
    }
}
