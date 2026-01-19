use crate::user::UserId;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Unique ID (UUID) for a single node within the file tree
pub type NodeId = u128;
#[derive(Serialize, Deserialize, Debug)]
pub enum NodeType {
    Folder,
    File,
    Link,
}

/// Unique ID (UUID) for a revision of a file
pub type RevisionId = u128;

/// The index of a chunk within a file
pub type ChunkIndex = u64;

pub type MetadataIv = [u8; 12];
pub type RevisionIv = [u8; 12];

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptedNode {
    pub id: NodeId,
    pub change_count: u64,
    pub parent_id: NodeId,
    pub owner_id: UserId,
    pub deleted_on: Option<NaiveDateTime>,
    pub node_type: NodeType,
    pub current_revision: Option<FileRevision>,
    pub encrypted_metadata: Vec<u8>,
    pub metadata_iv: MetadataIv,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileRevision {
    pub id: RevisionId,
    pub upload_ended_on: Option<NaiveDateTime>,
    pub upload_started_on: NaiveDateTime,
    pub iv: RevisionIv,
    pub chunk_count: u64,
}
