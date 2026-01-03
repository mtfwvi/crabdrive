use serde::{Deserialize, Serialize};

use crate::{data::DataAmount, user::UserType};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserGet200Response {
    username: String,
    user_type: UserType,
    storage_limit: Option<DataAmount>,
    created_on: u64,
    updated_on: u64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UserPost200Response {
    username: String,
    user_type: UserType,
    storage_limit: Option<DataAmount>,
    created_on: u64,
    updated_on: u64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UserUpdate200Response {
    username: String,
    user_type: UserType,
    storage_limit: Option<DataAmount>,
    created_on: u64,
    updated_on: u64,
}
