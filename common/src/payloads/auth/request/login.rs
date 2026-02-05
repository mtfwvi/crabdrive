use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostLoginRequest {
    pub username: String,
    pub password: String,
}
