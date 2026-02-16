use crate::encryption_key::EncryptionKey;
use crate::storage::{EncryptedNode, NodeId, ShareId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ShareEncryptionInfo {
    pub node_id: NodeId,
    pub wrapped_metadata_key: EncryptionKey
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SharedUserInfo {
    pub username: String
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostShareNodeResponse {
    NotFound,
    Ok(ShareId),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum  GetShareInfoResponse {
    Ok(ShareEncryptionInfo),
    NotFound
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GetNodeShareInfoResponse {
    Ok(Vec<SharedUserInfo>),
    NotFound
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostAcceptShareResponse {
    Ok,
    NotFound
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GetAcceptedSharedResponse {
    Ok(Vec<(EncryptionKey, EncryptedNode)>)
}

