use crate::storage::node::persistence::model::encrypted_metadata::EncryptedMetadata;
use crate::storage::node::persistence::model::node_entity::NodeEntity;
use crabdrive_common::node::NodeId;
use crabdrive_common::user::UserId;

pub(crate) trait NodeRepository {
    fn create_node(
        parent: Option<NodeId>,
        encrypted_metadata: EncryptedMetadata,
        owner: UserId,
        is_folder: bool,
    ) -> NodeId;

    fn update_node(node: NodeEntity) -> anyhow::Result<()>;

    fn update_metadata(node_id: NodeId, metadata: Vec<u8>) -> anyhow::Result<()>;

    fn move_to_trash(node_id: NodeId) -> anyhow::Result<()>;

    // this should either
    //  return a list of all nodes it deleted so that they associated chunks can be deleted
    // OR
    //  get access to the file repository and delete them
    fn purge_tree(node_id: NodeId) -> anyhow::Result<Vec<u64>>;

    // to move a node you need to change
    // - the node's store parent id
    // - the metadata of the old parent (remove the encryption key of the node we are moving)
    // - the metadata of the new parent (add the encryption key of the node we are moving)
    fn move_node(
        node_id: NodeId,
        from: NodeId,
        from_metadata: EncryptedMetadata,
        to: NodeId,
        to_metadata: EncryptedMetadata,
    ) -> anyhow::Result<()>;

    fn get_children(parent: NodeId) -> anyhow::Result<Vec<NodeEntity>>;
}
