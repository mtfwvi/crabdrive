use crabdrive_common::storage::{NodeId, RevisionId};

#[derive(Clone, Debug)]
pub struct TestNodeEntity {
    pub id: NodeId,
    pub active_revision: Option<TestRevisionEntity>,
}

#[derive(Clone, Debug)]
pub struct TestRevisionEntity {
    pub id: RevisionId,
    pub chunks: Vec<TestChunk>,
}

#[derive(Clone, Debug)]
pub struct TestChunk {
    pub checksum: String,
}
