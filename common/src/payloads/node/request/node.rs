use serde::{Deserialize, Serialize};
use crate::storage::MetadataIv;

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeDeleteBodyInfoPart {
    parent_change_count: u64,
    parent_metadata_iv: MetadataIv,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodePatchBodyInfoPart {
    node_change_count: u64,
    node_iv: MetadataIv,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct MoveNodeInfo {
    from_node_change_counter: u64,
    from_node_iv: MetadataIv,
    to_node_change_counter: u64,
    to_node_iv: MetadataIv,
}

pub type MoveNodePostBodyInfoPart = MoveNodeInfo;
pub type MoveNodeToTrashPostBodyInfoPart = MoveNodeInfo;
pub type MoveNodeOutOfTrashBodyInfoPart = MoveNodeInfo;
