use crate::user::UserId;
use chrono::NaiveDateTime;
use crate::iv::IV;
use crate::uuid::UUID;

use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    serialize::{self, IsNull, Output, ToSql},
    sql_types::Text,
    sqlite::Sqlite,
};

/// Unique ID (UUID) for a single node within the file tree
pub type NodeId = UUID;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "server", diesel(sql_type = Text))]
pub enum NodeType {
    Folder,
    File,
    Link,
}

#[cfg(feature = "server")]
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

#[cfg(feature = "server")]
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
pub type ChunkIndex = u32;

pub type MetadataIv = IV;
pub type RevisionIv = IV;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct EncryptedNode {
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct FileRevision {
    pub id: RevisionId,
    pub upload_ended_on: Option<NaiveDateTime>,
    pub upload_started_on: NaiveDateTime,
    pub iv: RevisionIv,
    pub chunk_count: u64,
}
