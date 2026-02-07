use crate::payloads::auth::response::login::UserKeys;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostRegisterRequest {
    pub username: String,
    pub password: String,
    pub keys: UserKeys,
}
