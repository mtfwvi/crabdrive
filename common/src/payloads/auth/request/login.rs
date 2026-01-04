use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostLoginRequest {
    username: String,
    password: String,
}