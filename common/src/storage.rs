use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use uuid::Uuid;

/// Unique ID (UUID) for a single node within the file tree
pub type NodeId = Uuid;
#[derive(Debug)]
pub enum NodeType {
    Folder,
    File,
    Link,
}

impl ToSql<Text, Sqlite> for NodeType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let value = match self {
            NodeType::Folder => "folder",
            NodeType::File => "file",
            NodeType::Link => "link",
        };

        out.set_value(value);
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Sqlite> for NodeType {
    fn from_sql(
        bytes: <Sqlite as diesel::backend::Backend>::RawValue<'_>,
    ) -> deserialize::Result<Self> {
        let s = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;

        match s.as_str() {
            "folder" => Ok(NodeType::Folder),
            "file" => Ok(NodeType::File),
            "link" => Ok(NodeType::Link),
            _ => Err(format!("Invalid NodeType: {}", s).into()),
        }
    }
}

/// Unique ID (UUID) for a revision of a file
pub type RevisionId = Uuid;

/// The index of a chunk within a file
pub type ChunkIndex = u64;
