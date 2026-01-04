use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostRegisterRequest {
    username: String,
    password: String,
}