use uuid::Uuid;
/// Unique ID (UUID) for a single node within the file tree
pub type NodeId = Uuid;
pub enum NodeType {
    Folder,
    File,
    Link,
}

/// Unique ID (UUID) for a revision of a file
pub type RevisionId = Uuid;

/// The index of a chunk within a file
pub type ChunkIndex = u64;
