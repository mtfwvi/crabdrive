use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostRegisterRequest {
    username: String,
    password: String,
}
