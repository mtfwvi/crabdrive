use crabdrive_common::user::UserId;
use serde::{Deserialize, Serialize};

use crate::user::SessionId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: UserId,
    /// A session which is associated with this
    pub session_id: SessionId,
    /// Unix timestamp when the token was **i**ssued **at**.
    pub iat: i64,
    /// Unix timestamp when the token **exp**ires.
    pub exp: i64,
    /// A unique ID for each JWT
    pub jti: String,
}
