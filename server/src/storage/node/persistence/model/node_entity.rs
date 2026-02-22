use chrono::NaiveDateTime;
use crabdrive_common::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::storage::RevisionId;
use crabdrive_common::storage::{NodeId, NodeType};
use crabdrive_common::uuid::UUID;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable,
    Selectable,
    Serialize,
    Deserialize,
    Debug,
    Insertable,
    AsChangeset,
    Clone,
    QueryableByName,
)]
#[diesel(table_name = crate::db::schema::Node)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(encryptionKey))]
pub(crate) struct NodeEntity {
    pub id: NodeId,
    pub parent_id: Option<NodeId>,
    pub owner_id: UUID,

    /// The metadata encrypted by the client
    ///
    /// It should contain
    /// - folder/file name
    /// - encryption keys of all child metadata
    /// - folder/file metadata (last accessed, ???)
    /// - additional info based on the node type (e.g. links should have a relative path to the node)
    pub metadata: EncryptedMetadata,

    /// The time the file was moved to the trash (None for a not deleted node)
    pub deleted_on: Option<NaiveDateTime>,

    /// Counter that indicates the amount of times the metadata was updated. The server should
    /// increment it during each metadata update. Before accepting a change from a client, the
    /// server should check that the counter indicated by the client matches and return an error if
    /// it doesn't. This avoids loosing data when two clients try to change the metadata
    /// simultaneously
    pub metadata_change_counter: i64,

    /// The revision of the file that is currently active (None for none-file nodes).
    /// May point to an incomplete revision when the file was just created and is being uploaded
    pub current_revision: Option<RevisionId>,

    pub node_type: NodeType,
}
