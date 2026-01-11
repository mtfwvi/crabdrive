use serde::{Deserialize, Serialize};

use crate::{data::DataAmount, user::UserType};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostUserRequest {
    username: String,
    password: String,
    user_type: UserType,
    storage_limit: Option<DataAmount>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PatchUserRequest {
    username: String,
    user_type: UserType,
    storage_limit: Option<DataAmount>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteUserRequest {}
