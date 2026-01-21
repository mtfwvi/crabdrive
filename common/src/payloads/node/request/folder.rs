use crate::storage::{MetadataIv, NodeId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostCreateFolderRequest {
    pub parent_metadata_iv: MetadataIv,
    pub parent_metadata_version: u64,
    pub parent_metadata: Vec<u8>,
    pub node_metadata_iv: MetadataIv,
    pub node_metadata: Vec<u8>,
    pub node_id: NodeId,
}
