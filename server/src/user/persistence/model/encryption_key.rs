use diesel::sqlite::Sqlite;
use diesel::deserialize::FromSql;
use diesel::serialize::ToSql;
use diesel::sql_types::Binary;
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
        let mut result: Vec<u8> = Vec::with_capacity(self.key.len() + 12);
        result.extend_from_slice(&self.iv); // Store IV
        result.extend_from_slice(&(self.key.len() as u16).to_be_bytes()); // Store length of IV to SQL
        result.extend_from_slice(&self.key); // Store Key
        out.set_value(result);
        Ok(diesel::serialize::IsNull::No)
    }
}

impl FromSql<Binary, Sqlite> for EncryptionKey {
    fn from_sql(
        mut bytes: <Sqlite as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let mut blob = bytes.read_blob();
        let mut iv_buf: IV = [0; 12];
        blob.read(&mut iv_buf)?;
        let mut key_len: [u8; 2] = [0; 2];
        blob.read(&mut key_len)?;
        let key_len = u16::from_be_bytes(key_len);
        let mut key_buf: Vec<u8> = vec![0; key_len.into()];
        blob.read(&mut key_buf)?;
        let ek = EncryptionKey {
            key: key_buf,
            iv: iv_buf,
        };
        Ok(ek)
    }
}
