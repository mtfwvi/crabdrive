use crabdrive_common::iv::IV;
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::ToSql;
use diesel::sql_types::Binary;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};
use std::io::Read;

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
        mut bytes: <Sqlite as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let mut blob = bytes.read_blob();
        let mut iv_buf: [u8; 12] = [0; 12];
        blob.read_exact(&mut iv_buf)?;
        let mut key_buf = Vec::with_capacity(blob.len() - iv_buf.len());
        blob.read_to_end(&mut key_buf)?;
        Ok(EncryptionKey {
            key: key_buf,
            iv: IV::new(iv_buf),
        })
    }
}
