use crabdrive_common::user::UserId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: UserId,

    /// unix timestamp (seconds since 1970)
    pub exp: i64,
}
