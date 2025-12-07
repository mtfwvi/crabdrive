use crate::storage::node::persistence::model::NodeId;

pub struct DataAmount {
    // TODO: Move to commons/model
    pub byte_count: u64,
}

pub type UserId = i32;

pub(crate) struct User {
    id: UserId,
    email: String,
    username: String,      // TODO: Both username and email? Discuss uses
    password_hash: String, // TODO: Correct? Anything else needed for auth?
    storage_limit: DataAmount,

    // TODO: What fields for encryption?
    // encrypted with key derived from user password
    master_key: Vec<u8>,
    master_key_iv: [u8; 12],

    // encrypted with master key
    private_key: Vec<u8>,
    private_key_iv: [u8; 12],

    // not encrypted (needs to be verified before each usage as the server could modify it
    public_key: Vec<u8>,

    // encrypted with master key
    // used to encrypt the users root folder metadata
    root_key: Vec<u8>,
    root_key_iv: [u8; 12],

    // should be created when the user first logs in
    root_node: Option<NodeId>,

    // encrypted with master key
    // used to encrypt the trash folder metadata
    trash_key: Vec<u8>,
    trash_key_iv: [u8; 12],

    // should be created when the user first logs in
    trash_node: Option<NodeId>,
}
