use crabdrive_common::iv::IV;
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::ToSql;
use diesel::sql_types::Binary;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FromSqlRow, AsExpression)]
#[diesel(sql_type = Binary)]
pub(crate) struct EncryptionKey {
    key: Vec<u8>,
    iv: IV,
}

impl ToSql<Binary, Sqlite> for EncryptionKey {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Sqlite>,
    ) -> diesel::serialize::Result {
        let mut out_buf = Vec::with_capacity(self.iv.len() + self.key.len());
        // First 12 bytes stores the IV
        out_buf.extend_from_slice(&self.iv.get());
        // Byte 13 - End stores the key
        out_buf.extend_from_slice(&self.key);
        out.set_value(out_buf);
        Ok(diesel::serialize::IsNull::No)
    }
}

impl FromSql<Binary, Sqlite> for EncryptionKey {
    fn from_sql(
        bytes: <Sqlite as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let blob = <Vec<u8> as FromSql<Binary, Sqlite>>::from_sql(bytes)?;
        let (iv_slice, key_slice) = blob.split_at(12);
        let mut iv_buf = [0u8; 12];
        iv_buf.copy_from_slice(iv_slice);
        Ok(EncryptionKey {
            key: key_slice.to_vec(),
            iv: IV::new(iv_buf),
        })
    }
}
