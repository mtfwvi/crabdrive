use crate::encryption_key::EncryptionKey;
use crate::storage::{EncryptedNode, NodeId, ShareId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ShareEncryptionInfo {
    pub node_id: NodeId,
    pub wrapped_metadata_key: EncryptionKey,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostShareNodeResponse {
    NotFound,
    Ok(ShareId),
    BadRequest(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GetAcceptShareInfoResponse {
    Ok(ShareEncryptionInfo),
    NotFound,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GetNodeShareInfo {
    Ok(ShareEncryptionInfo),
    NotFound,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum PostAcceptShareResponse {
    Ok,
    NotFound,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum GetAcceptedSharedResponse {
    Ok(Vec<(EncryptionKey, EncryptedNode)>),
}
