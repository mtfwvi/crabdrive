use crate::user::persistence::model::encryption_key::IV;
use diesel::deserialize::FromSql;
use diesel::serialize::ToSql;
use diesel::sql_types::Binary;
use diesel::sqlite::Sqlite;

use std::io::Read;

#[derive(Debug)]
pub(crate) struct EncryptedMetadata {
    data: Vec<u8>,
    iv: IV,
}

impl ToSql<Binary, Sqlite> for EncryptedMetadata {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Sqlite>,
    ) -> diesel::serialize::Result {
        let mut out_buf = Vec::with_capacity(self.iv.len() + self.data.len());
        // First 12 bytes stores the IV
        out_buf.extend_from_slice(&self.iv);
        // Byte 13 - End stores the encrypted metadata
        out_buf.extend_from_slice(&self.data);
        out.set_value(out_buf);
        Ok(diesel::serialize::IsNull::No)
    }
}

impl FromSql<Binary, Sqlite> for EncryptedMetadata {
    fn from_sql(
        mut bytes: <Sqlite as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let mut blob = bytes.read_blob();
        let mut iv_buf: IV = [0; 12];
        blob.read_exact(&mut iv_buf)?;
        let mut data_buf = Vec::with_capacity(blob.len() - iv_buf.len());
        blob.read_to_end(&mut data_buf)?;
        Ok(EncryptedMetadata {
            data: data_buf,
            iv: iv_buf,
        })
    }
}
