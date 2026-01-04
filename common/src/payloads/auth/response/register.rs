use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum PostRegisterResponse {
    Created,
    Unauthorized,
    Conflict(RegisterConflictReason),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RegisterConflictReason {
    UsernameTaken,
    IllegalUsername,
    OTHER,
}