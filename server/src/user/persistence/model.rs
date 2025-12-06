pub struct DataAmount {
    // TODO: Move to commons/model
    pub byte_count: u64,
}

pub(crate) struct User {
    id: i32,
    email: String,
    username: String,      // TODO: Both username and email? Discuss uses
    password_hash: String, // TODO: Correct? Anything else needed for auth?
    storage_limit: DataAmount,
    // TODO: What fields for encryption?
}
