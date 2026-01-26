use crate::encrypted_metadata::EncryptedMetadata;
use crate::storage::{NodeId, RevisionIv};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostCreateFileRequest {
    pub parent_metadata_version: u64,
    pub parent_metadata: EncryptedMetadata,
    pub node_metadata: EncryptedMetadata,
    // will be store in the revision
    // the server cannot trust it
    pub file_iv: RevisionIv,
    pub chunk_count: u64,
    pub node_id: NodeId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostUpdateFileRequest {
    file_iv: RevisionIv,
    chunk_count: u64,
}
