use crate::storage::MetadataIv;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostCreateFolderRequest {
    parent_metadata_iv: MetadataIv,
    parent_metadata_version: u64,
    parent_metadata: Vec<u8>,
    node_metadata_iv: MetadataIv,
    node_metadata: Vec<u8>,
}
