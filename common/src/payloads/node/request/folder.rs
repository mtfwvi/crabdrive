use serde::{Deserialize, Serialize};
use crate::storage::{MetadataIv, RevisionIv};

//TODO remove multipart
#[derive(Serialize, Deserialize, Debug)]
pub struct PostCreateFolderRequest {
    parent_metadata_iv: MetadataIv,
    parent_metadata_version: u64, //TODO add to openapi
    parent_metadata: Vec<u8>,
    node_metadata_iv: MetadataIv,
    node_metadata: Vec<u8>,
}