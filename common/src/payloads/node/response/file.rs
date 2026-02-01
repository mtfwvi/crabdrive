use crate::storage::{ChunkIndex, EncryptedNode, FileRevision};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum PostCreateFileResponse {
    Created(EncryptedNode),
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
    Ok(EncryptedNode),
    BadRequest(CommitFileError),
    NotFound,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CommitFileError {
    MissingChunks(Vec<ChunkIndex>),
    AlreadyCommitted
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GetVersionsResponse {
    Ok(Vec<FileRevision>),
    NotFound,
}
