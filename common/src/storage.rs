use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};

use crate::iv::IV;
use crate::uuid::UUID;

/// Unique ID (UUID) for a single node within the file tree
pub type NodeId = UUID;
#[derive(Debug, Serialize, Deserialize, FromSqlRow, AsExpression, Clone)]
#[diesel(sql_type = Text)]
pub enum NodeType {
    Folder,
    File,
    Link,
}

impl ToSql<Text, Sqlite> for NodeType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let value = match self {
            NodeType::Folder => "FOLDER",
            NodeType::File => "FILE",
            NodeType::Link => "LINK",
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
            "FOLDER" => Ok(NodeType::Folder),
            "FILE" => Ok(NodeType::File),
            "LINK" => Ok(NodeType::Link),
            _ => Err(format!("Invalid NodeType: {}", s).into()),
        }
    }
}

/// Unique ID (UUID) for a revision of a file
pub type RevisionId = UUID;

/// The index of a chunk within a file
pub type ChunkIndex = u64;

pub type MetadataIv = IV;
pub type RevisionIv = IV;
