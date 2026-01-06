use serde::{Deserialize, Serialize};

use crate::{data::DataAmount, user::UserType};

#[derive(Serialize, Deserialize, Debug)]
pub enum GetUserResponse {
    Ok(UserInfo),
    NotFound,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostUserResponse {
    Created(UserInfo),
    Conflict,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    username: String,
    user_type: UserType,
    storage_limit: Option<DataAmount>,
    created_on: u64,
    updated_on: u64,
}