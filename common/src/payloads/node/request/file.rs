use crate::encrypted_metadata::EncryptedMetadata;
use crate::storage::{ChunkIndex, NodeId, RevisionIv};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostCreateFileRequest {
    pub parent_metadata_version: i64,
    pub parent_metadata: EncryptedMetadata,
    pub node_metadata: EncryptedMetadata,
    // will be store in the revision
    // the server cannot trust it
    pub file_iv: RevisionIv,
    pub chunk_count: ChunkIndex,
    pub node_id: NodeId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostUpdateFileRequest {
    pub file_iv: RevisionIv,
    pub chunk_count: ChunkIndex,
}
