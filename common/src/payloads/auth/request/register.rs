use serde::{Deserialize, Serialize};

use crate::user::UserKeys;

#[derive(Serialize, Deserialize, Debug)]
pub struct PostRegisterRequest {
    pub username: String,
    pub password: String,
    pub invite_code: String,
    pub keys: UserKeys,
}
