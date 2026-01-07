use crate::storage::node::persistence::model::encrypted_metadata::EncryptedMetadata;
use chrono::NaiveDateTime;
use crabdrive_common::storage::RevisionId;
use crabdrive_common::storage::{NodeId, NodeType};
use crabdrive_common::uuid::UUID;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::db::schema::Node)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(encryptionKey))]
pub(crate) struct NodeEntity {
    id: NodeId,
    parent_id: Option<NodeId>,
    owner_id: UUID,

    /// The metadata encrypted by the client
    ///
    /// It should contain
    /// - folder/file name
    /// - encryption keys of all child metadata
    /// - folder/file metadata (last accessed, ???)
    /// - additional info based on the node type (e.g. links should have a relative path to the node)
    metadata: EncryptedMetadata,

    /// The time the file was moved to the trash (None for a not deleted node)
    deleted_on: Option<NaiveDateTime>,

    /// Counter that indicates the amount of times the metadata was updated. The server should
    /// increment it during each metadata update. Before accepting a change from a client, the
    /// server should check that the counter indicated by the client matches and return an error if
    /// it doesn't. This avoids loosing data when two clients try to change the metadata
    /// simultaneously
    metadata_change_counter: i64,

    /// The revision of the file that is currently active (None for none-file nodes).
    /// May point to an incomplete revision when the file was just created and is being uploaded
    current_revision: Option<RevisionId>,

    node_type: NodeType,
}
