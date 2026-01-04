use serde::{Deserialize, Serialize};
use crate::storage::MetadataIv;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteNodeRequest {
    parent_change_count: u64,
    parent_metadata_iv: MetadataIv,
    parent_node_metadata: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PatchNodeRequest {
    node_change_count: u64,
    node_iv: MetadataIv,
    node_metadata: Vec<u8>,
}

//TODO rename openapi object (MoveFile)
//TODO turn this into single json
#[derive(Serialize, Deserialize, Debug)]
pub struct MoveNodeData {
    from_node_change_counter: u64,
    from_node_iv: MetadataIv,
    from_node_metadata: Vec<u8>,
    to_node_change_counter: u64,
    to_node_iv: MetadataIv,
    to_node_metadata: Vec<u8>,
}

pub type PostMoveNodeRequest = MoveNodeData;
pub type PostMoveNodeToTrash = MoveNodeData;
pub type PostMoveNodeOutOfTrash = MoveNodeData;
