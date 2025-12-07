use crate::user::persistence::model::UserId;

pub type NodeId = u64;

pub struct Node {
    id: NodeId,
    parent_id: Option<NodeId>,
    owner_id: UserId,
    is_folder: bool,

    /// should contain
    /// - folder/file name
    /// - encryption keys of all child metadata
    /// - folder/file metadata (last accessed, ???)
    encrypted_metadata: Vec<u8>,
    iv: [u8; 12],

    // TODO decide if we should temporarily store deleted nodes in "trash" folder
    // this could be realized by setting the deleted boolean and an according timestamp
    // this also makes instantaneous restoring possible
    deleted: bool,
    deleted_on: Option<u64>,
}
