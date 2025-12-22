use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum RegisterConflictReason {
    UsernameTaken,
    IllegalUsername,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterPost201Response {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterPost401Response {
    // TODO decide if we want to send a reason
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterPost409Response {
    reason: RegisterConflictReason,
}