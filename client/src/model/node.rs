use crate::model::encryption::{ChildKey, EncryptionKey};
use chrono::NaiveDateTime;
use crabdrive_common::data::DataAmount;
use crabdrive_common::storage::{FileRevision, NodeId, NodeType};
use crabdrive_common::user::UserId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum NodeMetadata {
    V1(MetadataV1),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct MetadataV1 {
    pub name: String,
    pub last_modified: NaiveDateTime,
    pub created: NaiveDateTime,
    pub size: Option<DataAmount>,
    pub mime_type: Option<String>,

    /// used to encrypt chunks
    pub file_key: Option<EncryptionKey>,

    /// used to encrypt children
    pub children_key: Vec<ChildKey>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct DecryptedNode {
    pub id: NodeId,
    pub change_count: u64,
    pub parent_id: NodeId,
    pub owner_id: UserId,
    pub deleted_on: Option<NaiveDateTime>,
    pub node_type: NodeType,
    pub current_revision: Option<FileRevision>,
    pub metadata: NodeMetadata,
    pub encryption_key: EncryptionKey,
}