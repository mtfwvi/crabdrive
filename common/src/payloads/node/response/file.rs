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
    Ok(FileRevision),
    NotFound,
    BadRequest,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostCommitFileResponse {
    Ok(NodeInfo),
    BadRequest(Vec<u64>), // the missing parts
    NotFound,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GetVersionsResponse {
    Ok(Vec<FileRevision>),
    NotFound,
}
