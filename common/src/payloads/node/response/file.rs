use crate::storage::{ChunkIndex, EncryptedNode, FileRevision};
use serde::{Deserialize, Serialize};

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PostCreateFileResponse {
    Created(EncryptedNode) = 201,
    NotFound = 404,
    BadRequest = 400,
    Conflict = 409,
}

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PostUpdateFileResponse {
    Ok(FileRevision) = 200,
    NotFound = 404,
    BadRequest = 400,
}

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PostCommitFileResponse {
    Ok(EncryptedNode) = 200,
    BadRequest(Vec<ChunkIndex>) = 400, // the missing parts
    NotFound = 404,
}

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GetVersionsResponse {
    Ok(Vec<FileRevision>) = 200,
    NotFound = 404,
}
