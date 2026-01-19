use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::storage::MetadataIv;
use crate::{
    storage::{NodeId, NodeType, RevisionId, RevisionIv},
    user::UserId,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeInfo {
    pub id: NodeId,
    pub change_count: u64,
    pub parent_id: NodeId,
    pub owner_id: UserId,
    pub deleted_on: Option<NaiveDateTime>,
    pub node_type: NodeType,
    pub current_revision: Option<FileRevision>,
    pub encrypted_metadata: Vec<u8>,
    pub metadata_iv: MetadataIv,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileRevision {
    pub id: RevisionId,
    pub upload_ended_on: Option<NaiveDateTime>,
    pub upload_started_on: NaiveDateTime,
    pub iv: RevisionIv,
    pub chunk_count: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GetNodeResponse {
    Ok(NodeInfo),
    NotFound,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PatchNodeResponse {
    Ok(NodeInfo),
    NotFound,
    Conflict,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostMoveNodeResponse {
    Ok,
    NotFound,
    Conflict,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostMoveNodeToTrashResponse {
    Ok,
    NotFound,
    Conflict,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostMoveNodeOutOfTrashResponse {
    Ok,
    NotFound,
    Conflict,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DeleteNodeResponse {
    Ok,
    NotFound,
    Conflict,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GetPathBetweenNodesResponse {
    Ok(Vec<NodeInfo>),
    NoContent,
    NotFound,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GetNodeChildrenResponse {
    Ok(Vec<NodeInfo>),
    NotFound,
}
