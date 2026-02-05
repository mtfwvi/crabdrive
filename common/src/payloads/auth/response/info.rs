use crate::user::UserId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfUserInfo {
    pub username: String,
    pub user_id: UserId,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GetSelfInfoResponse {
    Ok(SelfUserInfo),
}
