use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshBody {
    pub bearer_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PostRefreshResponse {
    Ok(RefreshBody),
    Err
}
