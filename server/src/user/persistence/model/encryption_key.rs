use diesel::deserialize::FromSql;
use diesel::serialize::ToSql;
use diesel::sql_types::Binary;
use diesel::sqlite::Sqlite;
use std::io::Read;
// Initialization vector for encryption
pub type IV = [u8; 12];

#[derive(Debug)]
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
        out_buf.extend_from_slice(&self.iv);
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
        let mut iv_buf: IV = [0; 12];
        blob.read_exact(&mut iv_buf)?;
        let mut key_buf = Vec::with_capacity(blob.len() - iv_buf.len());
        blob.read_to_end(&mut key_buf)?;
        Ok(EncryptionKey {
            key: key_buf,
            iv: iv_buf,
        })
    }
}
