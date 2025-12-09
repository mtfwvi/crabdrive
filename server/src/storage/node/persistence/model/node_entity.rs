use crate::storage::node::persistence::model::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::node::NodeId;
use crabdrive_common::user::UserId;

pub struct NodeEntity {
    id: NodeId,
    parent_id: Option<NodeId>,
    owner_id: UserId,
    is_folder: bool,

    /// should contain
    /// - folder/file name
    /// - encryption keys of all child metadata
    /// - folder/file metadata (last accessed, ???)
    metadata: EncryptedMetadata,

    deleted: bool,
    deleted_on: Option<u64>,
}
