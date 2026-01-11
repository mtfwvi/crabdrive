use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::storage::MetadataIv;
use crate::{
    storage::{NodeId, NodeType, RevisionId, RevisionIv},
    user::UserId,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeInfo {
    id: NodeId,
    change_count: u64,
    parent_id: NodeId,
    owner_id: UserId,
    deleted_on: Option<NaiveDateTime>,
    node_type: NodeType,
    current_revision: Option<FileRevision>,
    encrypted_metadata: Vec<u8>,
    metadata_iv: MetadataIv,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileRevision {
    id: RevisionId,
    upload_ended_on: Option<NaiveDateTime>,
    upload_started_on: NaiveDateTime,
    iv: RevisionIv,
    chunk_count: u64,
}

impl FileRevision {
    pub fn new(
        id: RevisionId,
        upload_ended_on: Option<NaiveDateTime>,
        upload_started_on: NaiveDateTime,
        iv: RevisionIv,
        chunk_count: u64,
    ) -> Self {
        Self {
            id,
            upload_ended_on,
            upload_started_on,
            iv,
            chunk_count,
        }
    }
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
    NotFound, // one of the referenced nodes was not found
    Conflict, // version conflict (one of the nodes
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostMoveNodeToTrashResponse {
    Ok,
    NotFound, // one of the referenced nodes was not found
    Conflict, // version conflict (one of the nodes
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostMoveNodeOutOfTrashResponse {
    Ok,
    NotFound, // one of the referenced nodes was not found
    Conflict, // version conflict (one of the nodes
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
