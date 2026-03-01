use crate::data::DataAmount;
use crate::user::UserId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SelfUserInfo {
    pub username: String,
    pub user_id: UserId,
    pub storage_limit: DataAmount,
    pub storage_used: DataAmount,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GetSelfInfoResponse {
    Ok(SelfUserInfo),
}
