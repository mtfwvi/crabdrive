use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum PostLoginResponse {
    Ok(LoginSuccess),
    Unauthorized(LoginDeniedReason),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginSuccess {
    /// The jwt token signed by the server
    bearer_token: String,
    redirect_url: String,

    // the client should store the ids in its local storage since they do not change
    root_node_id: String,
    trash_node_id: String,

    should_initialize_encryption: bool,
}

impl LoginSuccess {
    pub fn new(
        bearer_token: String,
        redirect_url: String,
        root_node_id: String,
        trash_node_id: String,
        should_initialize_encryption: bool,
    ) -> Self {
        Self {
            bearer_token,
            redirect_url,
            root_node_id,
            trash_node_id,
            should_initialize_encryption,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LoginDeniedReason {
    Password,
    Username,
    OTHER,
}
