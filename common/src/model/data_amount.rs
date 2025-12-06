pub struct DataAmount {
    pub byte_count: u64,
}

impl DataAmount {
    pub fn of_bytes(byte_count: u64) -> DataAmount {
        DataAmount { byte_count }
    }
}
