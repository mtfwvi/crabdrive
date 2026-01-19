#[cfg(feature = "server")]
use std::convert::TryInto;

use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use diesel::{
    sqlite::Sqlite,
    sql_types::Binary,
    expression::AsExpression,
    serialize::{self, IsNull, Output, ToSql},
    deserialize::{self, FromSql, FromSqlRow},
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(
    FromSqlRow, AsExpression
))]
#[cfg_attr(feature = "server", diesel(sql_type = Binary))]
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

#[cfg(feature = "server")]
impl ToSql<Binary, Sqlite> for IV {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(self.0.as_slice());
        Ok(IsNull::No)
    }
}

#[cfg(feature = "server")]
impl FromSql<Binary, Sqlite> for IV {
    fn from_sql(bytes: <Sqlite as diesel::backend::Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let bytes_vec = Vec::<u8>::from_sql(bytes)?;
        let array: [u8; 12] = bytes_vec.try_into().map_err(|_| "IV not 12 bytes")?;
        Ok(IV(array))
    }
}
