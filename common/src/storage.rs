/// Unique ID (UUID) for a single node within the file tree
pub type NodeId = u128;
pub enum NodeType {
    Folder,
    File,
    Link,
}

/// Unique ID (UUID) for a revision of a file
pub type RevisionId = u128;

/// The index of a chunk within a file
pub type ChunkIndex = u64;
