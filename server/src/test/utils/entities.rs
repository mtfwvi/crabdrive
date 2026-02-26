use crabdrive_common::storage::{ChunkIndex, NodeId, NodeType, RevisionId};

#[derive(Clone, Debug)]
pub struct TestNodeEntity {
    pub id: NodeId,
    pub node_type: NodeType,
    pub children: Vec<TestNodeEntity>,
    pub active_revision: Option<TestRevisionEntity>,
}

#[derive(Clone, Debug)]
pub struct TestRevisionEntity {
    pub id: RevisionId,
    pub chunks: Vec<TestChunk>,
}

#[derive(Clone, Debug)]
pub struct TestChunk {
    pub index: ChunkIndex,
    pub checksum: String,
}
