use crate::encrypted_metadata::EncryptedMetadata;
use crate::storage::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostCreateFolderRequest {
    pub parent_metadata_version: u64,
    pub parent_metadata: EncryptedMetadata,
    pub node_metadata: EncryptedMetadata,
    pub node_id: NodeId,
}
