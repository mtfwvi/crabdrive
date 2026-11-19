use crate::http::AppState;
use crate::user::UserEntity;

use crabdrive_common::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::storage::{EncryptedNode, RevisionId};
use crabdrive_common::storage::{NodeId, NodeType};
use crabdrive_common::user::UserId;

use anyhow::Result;
use chrono::NaiveDateTime;
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
    Identifiable,
    Associations,
)]
#[diesel(table_name = crate::db::schema::Node)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(UserEntity, foreign_key = owner_id))]
pub struct NodeEntity {
    pub id: NodeId,
    pub parent_id: Option<NodeId>,
    pub owner_id: UserId,

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

impl NodeEntity {
    pub fn to_encrypted_node(&self, state: &AppState) -> Result<EncryptedNode> {
        Ok(EncryptedNode {
            id: self.id,
            change_count: self.metadata_change_counter,
            parent_id: self.parent_id,
            owner_id: self.owner_id,
            deleted_on: self.deleted_on,
            node_type: self.node_type,
            current_revision: match self.current_revision {
                Some(id) => {
                    let entity = state
                        .revision_repository
                        .get_revision(id)?
                        .expect("Inconsistent data between node and revision");
                    Some(entity.to_file_revision())
                }
                None => None,
            },
            encrypted_metadata: self.metadata.clone(),
            has_access: state.node_repository.get_access_list(self.id)?,
        })
    }
}
