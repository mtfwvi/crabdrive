use serde::{Deserialize, Serialize};

use crate::storage::EncryptedNode;

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GetNodeResponse {
    Ok(EncryptedNode) = 200,
    NotFound = 404,
}

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PatchNodeResponse {
    Ok(EncryptedNode) = 200,
    NotFound = 404,
    Conflict = 409,
}

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum PostMoveNodeResponse {
    Ok = 200,
    NotFound = 404,
    Conflict = 409,
}

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum PostMoveNodeToTrashResponse {
    Ok = 200,
    NotFound = 404,
    Conflict = 409,
}

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum PostMoveNodeOutOfTrashResponse {
    Ok = 200,
    NotFound = 404,
    Conflict = 409,
}

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum DeleteNodeResponse {
    Ok = 200,
    NotFound = 404,
    Conflict = 409,
}

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GetPathBetweenNodesResponse {
    Ok(Vec<EncryptedNode>) = 200,
    NoContent = 204,
    NotFound = 404,
}

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GetNodeChildrenResponse {
    Ok(Vec<EncryptedNode>) = 200,
    NotFound = 404,
}
