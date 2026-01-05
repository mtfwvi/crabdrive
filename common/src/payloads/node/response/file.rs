use crate::payloads::node::response::node::{FileRevision, NodeInfo};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum PostCreateFileResponse {
    Created(NodeInfo), // TODO change response code
    NotFound,
    BadRequest,
    Conflict,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostUpdateFileResponse {
    Success(FileRevision),
    NotFound,
    BadRequest, //TODO add to openapi
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostCommitFileResponse {
    Success(NodeInfo),
    NotFound,
}