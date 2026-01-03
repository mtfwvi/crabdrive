use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{storage::{NodeId, NodeType, RevisionId, RevisionIv}, user::UserId};

#[derive(Serialize, Deserialize, Debug)] 
pub struct Node {
    id: NodeId,
    change_count: u64,
    parent_id: NodeId,
    owner_id: UserId,
    deleted_on: Option<NaiveDateTime>,
    node_type: NodeType,
    current_revision: Option<FileRevision>
}

#[derive(Serialize, Deserialize, Debug)] 
pub struct FileRevision {
    id: RevisionId,
    upload_ended_on: Option<NaiveDateTime>,
    upload_started_on: NaiveDateTime,
    iv: RevisionIv,
    chunk_count: u64,
}


pub type NodeGet200ResponseInfoPart = Node;
