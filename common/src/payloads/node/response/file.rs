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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum PostCommitFileResponse {
    Ok(EncryptedNode),
    BadRequest(CommitFileError),
    NotFound,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum CommitFileError {
    MissingChunks(Vec<ChunkIndex>),
    AlreadyCommitted,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GetVersionsResponse {
    Ok(Vec<FileRevision>),
    NotFound,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GetRevisionHistoryResponse {
    Ok(Vec<RevisionInfo>),
    NotFound,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RevisionInfo {
    pub id: String,
    pub upload_started: String,
    pub upload_completed: Option<String>,
    pub is_current: bool,
    pub chunk_count: i64,
}
