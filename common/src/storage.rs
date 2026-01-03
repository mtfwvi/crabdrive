use uuid::Uuid;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use rusqlite::Result;

/// Unique ID (UUID) for a single node within the file tree
pub type NodeId = Uuid;
pub enum NodeType {
    Folder,
    File,
    Link,
}

impl ToSql for NodeType {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let s = match self {
            NodeType::Folder => "folder",
            NodeType::File => "file",
            NodeType::Link => "link",
        };
        Ok(ToSqlOutput::from(s))
    }
}

impl FromSql for NodeType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Text(s) => {
                let s = std::str::from_utf8(s)
                    .map_err(|e| FromSqlError::Other(Box::new(e)))?;
                match s {
                    "folder" => Ok(NodeType::Folder),
                    "file" => Ok(NodeType::File),
                    "link" => Ok(NodeType::Link),
                    _ => Err(FromSqlError::Other(
                        format!("Invalid NodeType: {}", s).into()
                    )),
                }
            }
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

/// Unique ID (UUID) for a revision of a file
pub type RevisionId = Uuid;

/// The index of a chunk within a file
pub type ChunkIndex = u64;
