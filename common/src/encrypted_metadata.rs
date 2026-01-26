use crate::iv::IV;

#[cfg(feature = "server")]
use diesel::{
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    serialize::ToSql,
    sql_types::Binary,
    sqlite::Sqlite,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "server", diesel(sql_type = Binary))]
pub struct EncryptedMetadata {
    data: Vec<u8>,
    iv: IV,
}

impl EncryptedMetadata {
    pub fn nil() -> Self {
        EncryptedMetadata {
            data: vec![0, 0, 0, 0],
            iv: IV::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        }
    }

    pub fn metadata(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn iv(&self) -> &IV {
        &self.iv
    }

    pub fn new(data: Vec<u8>, iv: IV) -> Self {
        Self { data, iv }
    }
}

#[cfg(feature = "server")]
impl ToSql<Binary, Sqlite> for EncryptedMetadata {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Sqlite>,
    ) -> diesel::serialize::Result {
        let mut out_buf = Vec::with_capacity(self.iv.len() + self.data.len());
        // First 12 bytes stores the IV
        out_buf.extend_from_slice(&self.iv.get());
        // Byte 13 - End stores the encrypted metadata
        out_buf.extend_from_slice(&self.data);
        out.set_value(out_buf);
        Ok(diesel::serialize::IsNull::No)
    }
}

#[cfg(feature = "server")]
impl FromSql<Binary, Sqlite> for EncryptedMetadata {
    fn from_sql(
        bytes: <Sqlite as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let blob = <Vec<u8> as FromSql<Binary, Sqlite>>::from_sql(bytes)?;
        let (iv_slice, data_slice) = blob.split_at(12);
        let mut iv_buf = [0u8; 12];
        iv_buf.copy_from_slice(iv_slice);
        Ok(EncryptedMetadata {
            data: data_slice.to_vec(),
            iv: IV::new(iv_buf),
        })
    }
}

//write tests for those two functions
