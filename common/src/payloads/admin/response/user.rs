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
    pub username: String,
    pub user_type: UserType,
    pub storage_limit: Option<DataAmount>,
    pub created_on: NaiveDateTime,
    pub updated_on: NaiveDateTime,
}
