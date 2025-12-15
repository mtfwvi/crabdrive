use crate::storage::node::persistence::model::encrypted_metadata::EncryptedMetadata;
use crate::storage::node::persistence::model::node_entity::NodeEntity;
use anyhow::Result;
use crabdrive_common::storage::NodeId;
use crabdrive_common::user::UserId;

pub(crate) trait NodeRepository {
    fn create_node(
        &self,
        parent: Option<NodeId>,
        encrypted_metadata: EncryptedMetadata,
        owner: UserId,
        is_folder: bool,
    ) -> NodeId;

    fn get_node(&self, node_id: NodeId) -> Result<NodeEntity>;

    fn update_node(&self, node: NodeEntity) -> Result<()>;

    /// Returns a list of all nodes it deleted so that the associated chunks can be deleted
    fn purge_tree(&self, node_id: NodeId) -> Result<NodeEntity>;

    /// Move a node from one parent to another. Requires:
    /// - the id of the node to move
    /// - the metadata of the old parent (remove the encryption key of the node we are moving)
    /// - the metadata of the new parent (add the encryption key of the node we are moving)
    fn move_node(
        &self,
        node_id: NodeId,
        from: NodeId,
        from_metadata: EncryptedMetadata,
        to: NodeId,
        to_metadata: EncryptedMetadata,
    ) -> Result<()>;

    fn get_children(&self, parent_id: NodeId) -> Result<Vec<NodeEntity>>;
}
