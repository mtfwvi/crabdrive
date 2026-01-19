use serde::{Deserialize, Serialize};

/// Unique ID (UUID) for a user
pub type UserId = u128;

#[derive(Deserialize, Serialize, Debug)]
pub enum UserType {
    User,
    Admin,
    Restricted,
}
