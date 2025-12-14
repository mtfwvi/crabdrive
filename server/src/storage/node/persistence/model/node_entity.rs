use crate::storage::node::persistence::model::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::node::{NodeId, NodeType};
use crabdrive_common::user::UserId;
use crabdrive_common::version::VersionId;

pub struct NodeEntity {
    id: NodeId,
    parent_id: Option<NodeId>,
    owner_id: UserId,

    /// The metadata encrypted by the client
    ///
    /// It should contain
    /// - folder/file name
    /// - encryption keys of all child metadata
    /// - folder/file metadata (last accessed, ???)
    /// - additional info based on the node type (e.g. links should have a relative path to the node)
    metadata: EncryptedMetadata,

    /// The time the file was moved to the trash as unix time (None for a not deleted node)
    deleted_on: Option<u64>,

    /// Counter that indicates the amount of times the metadata was updated. The server should
    /// increment it during each metadata update. Before accepting a change from a client, the
    /// server should check that the counter indicated by the client matches and return an error if
    /// it doesn't. This avoids loosing data when two clients try to change the metadata
    /// simultaneously
    metadata_change_counter: u64,

    /// The version of the file that is currently active (None for none-file nodes).
    /// May point to an incomplete version when the file was just created and is being uploaded
    current_file_version: Option<VersionId>,

    //TODO find out how to store this in the database (string or int)
    node_type: NodeType,
}
