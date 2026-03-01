use serde::{Deserialize, Serialize};

use crate::storage::EncryptedNode;
#[derive(Serialize, Deserialize, Debug)]
pub enum GetNodeResponse {
    Ok(EncryptedNode),
    NotFound,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PatchNodeResponse {
    Ok(EncryptedNode),
    NotFound,
    Conflict,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostMoveNodeResponse {
    Ok,
    BadRequest,
    NotFound,
    Conflict,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostMoveNodeToTrashResponse {
    Ok,
    BadRequest,
    NotFound,
    Conflict,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostMoveNodeOutOfTrashResponse {
    Ok,
    BadRequest,
    NotFound,
    Conflict,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DeleteNodeResponse {
    Ok,
    NotFound,
    Conflict,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum GetAccessiblePathResponse {
    Ok(Vec<EncryptedNode>),
    NotFound,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GetNodeChildrenResponse {
    Ok(Vec<EncryptedNode>),
    BadRequest,
    NotFound,
}
