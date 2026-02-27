use crate::db::connection::DbPool;
use crate::db::operations::node::*;
use crate::db::operations::share::*;

use crate::storage::node::NodeEntity;

use crabdrive_common::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::storage::{NodeId, NodeType};
use crabdrive_common::user::UserId;

use std::sync::Arc;

use anyhow::{Context, Ok, Result};

pub(crate) trait NodeRepository {
    fn create_node(
        &self,
        parent: Option<NodeId>,
        encrypted_metadata: EncryptedMetadata,
        owner: UserId,
        node_type: crabdrive_common::storage::NodeType,
        node_id: NodeId,
    ) -> Result<NodeEntity>;

    fn get_node(&self, id: NodeId) -> Result<Option<NodeEntity>>;

    fn update_node(&self, node: &NodeEntity) -> Result<NodeEntity>;

    /// Returns a list of all nodes it deleted so that the associated chunks can be deleted
    fn purge_tree(&self, id: NodeId) -> Result<Vec<NodeEntity>>;

    /// Move a node from one parent to another. Requires:
    /// - the id of the node to move
    /// - the metadata of the old parent (remove the encryption key of the node we are moving)
    /// - the metadata of the new parent (add the encryption key of the node we are moving)
    fn move_node(
        &self,
        id: NodeId,
        from: NodeId,
        from_metadata: EncryptedMetadata,
        to: NodeId,
        to_metadata: EncryptedMetadata,
    ) -> Result<()>;

    /// Get all children of a node
    fn get_children(&self, parent_id: NodeId) -> Result<Vec<NodeEntity>>;

    /// Get the node entities of the path between to nodes
    fn get_path_between_nodes(&self, from: NodeId, to: NodeId) -> Result<Option<Vec<NodeEntity>>>;

    /// Check if a user has access to the node (own nodes & shared nodes)
    fn has_access(&self, id: NodeId, user: UserId) -> Result<bool>;

    /// Get the path from a node to the root or trash node
    fn get_path_to_root(&self, node: NodeId) -> Result<Vec<NodeEntity>>;

    /// Get a list of tuples `(UserId, Username)`, on which users have access to a node
    fn get_access_list(&self, node: NodeId) -> Result<Vec<(UserId, String)>>;
}

pub struct NodeState {
    db_pool: Arc<DbPool>,
}

impl NodeState {
    pub fn new(db_pool: Arc<DbPool>) -> Self {
        Self { db_pool }
    }
}

impl NodeRepository for NodeState {
    fn create_node(
        &self,
        parent: Option<NodeId>,
        encrypted_metadata: EncryptedMetadata,
        owner: UserId,
        node_type: NodeType,
        node_id: NodeId,
    ) -> Result<NodeEntity> {
        let mut conn = self.db_pool.get()?;

        let node = NodeEntity {
            id: node_id,
            parent_id: parent,
            owner_id: owner,
            metadata: encrypted_metadata.clone(),
            deleted_on: None,
            metadata_change_counter: 0,
            current_revision: None,
            node_type,
        };

        if let Some(parent_id) = parent {
            let parent_node = select_node(&mut conn, parent_id)
                .context("Failed to select parent node")?
                .context("Parent node not found")?;

            insert_node(&mut conn, &node, &parent_node.metadata)
                .context("Failed to insert node")?;
        } else {
            insert_node(&mut conn, &node, &encrypted_metadata)
                .context("Failed to insert root node")?;
        }

        Ok(node)
    }

    fn get_node(&self, id: NodeId) -> Result<Option<NodeEntity>> {
        let mut conn = self.db_pool.get()?;
        select_node(&mut conn, id).context("Failed to select node")
    }

    fn update_node(&self, node: &NodeEntity) -> Result<NodeEntity> {
        let mut conn = self.db_pool.get()?;
        update_node(&mut conn, node, None)
            .map_err(|e| anyhow::anyhow!("{}", e))
            .context("Failed to update node")
    }

    fn purge_tree(&self, id: NodeId) -> Result<Vec<NodeEntity>> {
        let mut deleted_nodes = Vec::new();

        fn delete_recursively(
            db_pool: &DbPool,
            node_id: NodeId,
            deleted_nodes: &mut Vec<NodeEntity>,
        ) -> Result<()> {
            let mut conn = db_pool.get()?;

            let children =
                get_all_children(&mut conn, node_id).context("Failed to get children")?;

            for child in children {
                delete_recursively(db_pool, child.id, deleted_nodes)?;
            }

            let node = select_node(&mut conn, node_id)
                .context("Failed to select node")?
                .context("Node not found")?;

            if let Some(parent_id) = node.parent_id {
                let parent_node = select_node(&mut conn, parent_id)
                    .context("Failed to select parent")?
                    .context("Parent not found")?;

                let deleted_node = delete_node(&mut conn, node_id, &parent_node.metadata)
                    .context("Failed to delete node")?;
                deleted_nodes.push(deleted_node);
            } else {
                let empty_metadata = node.metadata.clone();
                let deleted_node = delete_node(&mut conn, node_id, &empty_metadata)
                    .context("Failed to delete root node")?;
                deleted_nodes.push(deleted_node);
            }

            Ok(())
        }

        delete_recursively(&self.db_pool, id, &mut deleted_nodes)?;

        Ok(deleted_nodes)
    }

    fn move_node(
        &self,
        id: NodeId,
        from: NodeId,
        from_metadata: EncryptedMetadata,
        to: NodeId,
        to_metadata: EncryptedMetadata,
    ) -> Result<()> {
        let mut conn = self.db_pool.get()?;
        move_node(&mut conn, id, from, from_metadata, to, to_metadata)
    }

    fn get_children(&self, parent_id: NodeId) -> Result<Vec<NodeEntity>> {
        let mut conn = self.db_pool.get()?;
        get_all_children(&mut conn, parent_id).context("Failed to get children")
    }

    fn get_path_between_nodes(&self, from: NodeId, to: NodeId) -> Result<Option<Vec<NodeEntity>>> {
        let mut conn = self.db_pool.get()?;
        let path: Vec<NodeEntity> = get_path_between_nodes(&mut conn, from, to)?;

        if path[0].id != from {
            Ok(None)
        } else {
            Ok(Some(path))
        }
    }

    fn has_access(&self, id: NodeId, user: UserId) -> Result<bool> {
        let mut conn = self.db_pool.get()?;
        has_access(&mut conn, id, user)
    }

    fn get_path_to_root(&self, node: NodeId) -> Result<Vec<NodeEntity>> {
        let mut conn = self.db_pool.get()?;
        // since there is no node with the nil uuid (hopefully) it returns the path from a root node to the node
        get_path_between_nodes(&mut conn, NodeId::nil(), node)
    }

    fn get_access_list(&self, node: NodeId) -> Result<Vec<(UserId, String)>> {
        let mut conn = self.db_pool.get()?;
        get_access_list_parent_tree(&mut conn, node)
    }
}
