use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub enum PostRegisterResponse {
    Created,
    Unauthorized,
    Conflict(RegisterConflictReason),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub enum RegisterConflictReason {
    UsernameTaken,
    IllegalUsername,
    OTHER,
}
