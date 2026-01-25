#[cfg(feature = "server")]
use std::convert::TryInto;

use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    serialize::{self, IsNull, Output, ToSql},
    sql_types::Binary,
    sqlite::Sqlite,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(FromSqlRow, AsExpression))]
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

    /// used to encrypt chunks to prevent chunk reordering
    // 2^32 chunks is enough as this allows you to have a max file size of 2^32 * 16 MB (chunk size)
    pub fn prefix_with_u32(&self, prefix: u32) -> IvWithPrefix {
        let prefix_bytes = prefix.to_be_bytes();

        let mut full_iv: [u8; 16] = [0; 16];

        full_iv[..4].clone_from_slice(&prefix_bytes[..]);
        full_iv[4..].clone_from_slice(&self.0[..]);

        full_iv
    }
}

pub type IvWithPrefix = [u8; 16];

#[cfg(feature = "server")]
impl ToSql<Binary, Sqlite> for IV {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(self.0.as_slice());
        Ok(IsNull::No)
    }
}

#[cfg(feature = "server")]
impl FromSql<Binary, Sqlite> for IV {
    fn from_sql(
        bytes: <Sqlite as diesel::backend::Backend>::RawValue<'_>,
    ) -> deserialize::Result<Self> {
        let bytes_vec = Vec::<u8>::from_sql(bytes)?;
        let array: [u8; 12] = bytes_vec.try_into().map_err(|_| "IV not 12 bytes")?;
        Ok(IV(array))
    }
}

#[cfg(test)]
mod test {
    use crate::iv::IV;
    use test_case::test_case;

    #[test_case(u32::MAX, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], [255, 255, 255, 255, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]; "test_prefix_iv1")]
    #[test_case(258, [1, 2, 3, 4, 5, 6, 25, 8, 9, 10, 11, 12],  [0, 0, 1, 2, 1, 2, 3, 4, 5, 6, 25, 8, 9, 10, 11, 12]; "test_prefix_iv2")]
    fn test_prefix_iv(prefix: u32, iv: [u8;12], expected: [u8; 16]) {
        let iv = IV::new(iv);
        let iv_with_prefix = iv.prefix_with_u32(prefix);

        assert_eq!(iv_with_prefix, expected);
    }
}
