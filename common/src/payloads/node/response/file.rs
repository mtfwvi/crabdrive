use crate::payloads::node::response::node::{FileRevision, NodeInfo};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum PostCreateFileResponse {
    Created(NodeInfo),
    NotFound,
    BadRequest,
    Conflict,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostUpdateFileResponse {
    Success(FileRevision),
    NotFound,
    BadRequest,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostCommitFileResponse {
    Success(NodeInfo),
    BadRequest(Vec<u64>), // the missing parts
    NotFound,
}