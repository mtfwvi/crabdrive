use crate::model::encryption::{ChildKey, FileKey, MetadataKey};
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
    pub file_key: Option<FileKey>,

    /// used to encrypt children
    pub children_key: Vec<ChildKey>,
}

impl NodeMetadata {
    pub(crate) fn v1(
        name: String,
        last_modified: NaiveDateTime,
        created: NaiveDateTime,
        size: Option<DataAmount>,
        mime_type: Option<String>,
        file_key: Option<FileKey>,
        children_key: Vec<ChildKey>,
    ) -> Self {
        NodeMetadata::V1(MetadataV1 {
            name,
            last_modified,
            created,
            size,
            mime_type,
            file_key,
            children_key,
        })
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct DecryptedNode {
    pub id: NodeId,
    pub change_count: i64,
    pub parent_id: Option<NodeId>,
    pub owner_id: UserId,
    pub deleted_on: Option<NaiveDateTime>,
    pub node_type: NodeType,
    pub current_revision: Option<FileRevision>,
    pub metadata: NodeMetadata,
    /// The metadata encryption key for this node
    pub encryption_key: MetadataKey,
}
