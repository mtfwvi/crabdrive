use serde::{Deserialize, Serialize};

use crate::user::UserKeys;

#[derive(Serialize, Deserialize, Debug)]
pub struct PostRegisterRequest {
    pub username: String,
    pub password: String,
    pub keys: UserKeys,
}
