use crate::storage::EncryptedNode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum PostCreateFolderResponse {
    Created(EncryptedNode),
    NotFound,
    BadRequest,
    Conflict,
}
