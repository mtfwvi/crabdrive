use crate::storage::EncryptedNode;
use serde::{Deserialize, Serialize};

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PostCreateFolderResponse {
    Created(EncryptedNode) = 201,
    NotFound = 404,
    BadRequest = 400,
    Conflict = 409,
}
