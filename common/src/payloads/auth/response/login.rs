use crate::storage::NodeId;
use crate::user::UserKeys;
use serde::{Deserialize, Serialize};

#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Deserialize, Debug)]
pub enum PostLoginResponse {
    Ok(LoginSuccess),
    Unauthorized(LoginDeniedReason),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginSuccess {
    /// The jwt token signed by the server
    pub bearer_token: String,
    pub redirect_url: String,

    // the client should store the ids in its local storage since they do not change
    pub root_node_id: NodeId,
    pub trash_node_id: NodeId,

    pub should_initialize_encryption: bool,

    // option in case they have not been initialized (impossible currently but may be useful with admin panel)
    pub user_keys: Option<UserKeys>,
}

impl LoginSuccess {
    pub fn new(
        bearer_token: String,
        redirect_url: String,
        root_node_id: NodeId,
        trash_node_id: NodeId,
        should_initialize_encryption: bool,
        encrypted_user_keys: Option<UserKeys>,
    ) -> Self {
        Self {
            bearer_token,
            redirect_url,
            root_node_id,
            trash_node_id,
            should_initialize_encryption,
            user_keys: encrypted_user_keys,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LoginDeniedReason {
    Password,
    Username,
    OTHER,
}
