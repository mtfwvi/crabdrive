use serde::{Deserialize, Serialize};
use std::fmt::Display;

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

impl Display for RegisterConflictReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UsernameTaken => write!(f, "Username taken"),
            Self::IllegalUsername => write!(f, "Illegal username"),
            Self::OTHER => write!(f, "Unknown reason"),
        }
    }
}
