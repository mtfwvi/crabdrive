use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterPostBody {
    username: String,
    password: String,
}