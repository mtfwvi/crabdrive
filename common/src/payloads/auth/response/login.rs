use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum LoginDeniedReason {
    Password,
    Username,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginPost200Response {
    /// The jwt token signed by the server
    bearer_token: String,
    redirect_url: String,

    // the client should store the ids in its local storage since they do not change
    root_node_id: String,
    trash_node_id: String,
    
    should_initialize_encryption: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginPost401Response {
    reason: LoginDeniedReason,
}