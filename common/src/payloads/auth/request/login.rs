use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostLoginRequest {
    username: String,
    password: String,
}
