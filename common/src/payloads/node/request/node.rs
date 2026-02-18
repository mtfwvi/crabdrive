use crate::encrypted_metadata::EncryptedMetadata;
use crate::storage::NodeId;
use serde::{Deserialize, Serialize};

// used to parse the query parameters in the get_path_between nodes handler
#[derive(Serialize, Deserialize, Debug)]
pub struct PathConstraints {
    pub from_id: NodeId,
    pub to_id: NodeId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteNodeRequest {
    pub parent_change_count: i64,
    pub parent_node_metadata: EncryptedMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PatchNodeRequest {
    pub node_change_count: i64,
    pub node_metadata: EncryptedMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveNodeData {
    pub from_node_change_counter: i64,
    pub from_node_metadata: EncryptedMetadata,
    pub to_node_change_counter: i64,
    pub to_node_metadata: EncryptedMetadata,
    pub to_node_id: NodeId,
}

pub type PostMoveNodeRequest = MoveNodeData;
pub type PostMoveNodeToTrashRequest = MoveNodeData;
pub type PostMoveNodeOutOfTrashRequest = MoveNodeData;

#[derive(Serialize, Deserialize, Debug)]
pub struct EmptyTrashRequest {
    pub older_than_days: i64,
}
