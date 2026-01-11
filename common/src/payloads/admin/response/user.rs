use chrono::NaiveDateTime;
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
pub enum DeleteUserResponse {
    Ok(UserInfo),
    NotFound,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PatchUserResponse {
    Ok(UserInfo),
    NotFound,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    username: String,
    user_type: UserType,
    storage_limit: Option<DataAmount>,
    created_on: NaiveDateTime,
    updated_on: u64,
}
