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

    pub fn prefix_with_u32(&self, prefix: u32) -> [u8; 16] {
        let prefix_bytes = prefix.to_be_bytes();

        let mut full_iv: [u8; 16] = [0; 16];

        full_iv[..4].clone_from_slice(&prefix_bytes[..]);
        full_iv[4..].clone_from_slice(&self.0[..]);

        full_iv
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

mod test {
    use crate::iv::IV;

    #[test]
    fn test_prefix_iv1() {
        let iv = IV::new([1,2,3,4,5,6,7,8,9,10,11,12]);
        let prefix = u32::MAX;

        let iv_with_prefix = iv.prefix_with_u32(prefix);
        assert_eq!(iv_with_prefix, [255, 255, 255, 255, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }

    #[test]
    fn test_prefix_iv2() {
        let iv = IV::new([1,2,3,4,5,6,25,8,9,10,11,12]);
        let prefix = 258;

        let iv_with_prefix = iv.prefix_with_u32(prefix);
        assert_eq!(iv_with_prefix, [0, 0, 1, 2, 1, 2, 3, 4, 5, 6, 25, 8, 9, 10, 11, 12]);
    }
}
