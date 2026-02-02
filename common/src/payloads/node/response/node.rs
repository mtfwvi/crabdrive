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
    NotFound,
    Conflict,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostMoveNodeToTrashResponse {
    Ok,
    NotFound,
    Conflict,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostMoveNodeOutOfTrashResponse {
    Ok,
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
pub enum GetPathBetweenNodesResponse {
    Ok(Vec<EncryptedNode>),
    NoContent,
    NotFound,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GetNodeChildrenResponse {
    Ok(Vec<EncryptedNode>),
    NotFound,
}
