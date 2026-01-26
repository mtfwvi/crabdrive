use crate::encrypted_metadata::EncryptedMetadata;
use crate::storage::NodeId;
use serde::{Deserialize, Serialize};

// used to parse the query parameters in the get_path_between nodes handler
#[derive(Serialize, Deserialize, Debug)]
pub struct PathConstraints {
    from_id: NodeId,
    to_id: NodeId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteNodeRequest {
    parent_change_count: u64,
    parent_node_metadata: EncryptedMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PatchNodeRequest {
    node_change_count: u64,
    node_metadata: EncryptedMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveNodeData {
    from_node_change_counter: u64,
    from_node_metadata: EncryptedMetadata,
    to_node_change_counter: u64,
    to_node_metadata: EncryptedMetadata,
    to_node_id: NodeId,
}

pub type PostMoveNodeRequest = MoveNodeData;
pub type PostMoveNodeToTrashRequest = MoveNodeData;
pub type PostMoveNodeOutOfTrashRequest = MoveNodeData;
