use crate::db::connection::DbPool;
use crate::db::operations;
use crate::db::operations::{delete_node, get_all_children, insert_node, select_node, update_node};
use crate::storage::node::persistence::model::node_entity::NodeEntity;
use anyhow::{Context, Ok, Result};
use crabdrive_common::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::storage::NodeId;
use crabdrive_common::storage::NodeType;
use crabdrive_common::uuid::UUID;
use std::sync::Arc;

pub(crate) trait NodeRepository {
    fn create_node(
        &self,
        parent: Option<NodeId>,
        encrypted_metadata: EncryptedMetadata,
        owner: UUID,
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

    fn get_children(&self, parent_id: NodeId) -> Result<Vec<NodeEntity>>;

    fn move_node_to_trash(
        &self,
        id: NodeId,
        from: NodeId,
        from_metadata: EncryptedMetadata,
        to_trash: NodeId,
        to_trash_metadata: EncryptedMetadata,
    ) -> Result<()>;

    fn move_node_out_of_trash(
        &self,
        id: NodeId,
        from_trash: NodeId,
        from_trash_metadata: EncryptedMetadata,
        to: NodeId,
        to_metadata: EncryptedMetadata,
    ) -> Result<()>;
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
        owner: UUID,
        node_type: NodeType,
        node_id: NodeId,
    ) -> Result<NodeEntity> {
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
            let parent_node = select_node(&self.db_pool, parent_id)
                .context("Failed to select parent node")?
                .context("Parent node not found")?;

            insert_node(&self.db_pool, &node, &parent_node.metadata)
                .context("Failed to insert node")?;
        } else {
            insert_node(&self.db_pool, &node, &encrypted_metadata)
                .context("Failed to insert root node")?;
        }

        Ok(node)
    }

    fn get_node(&self, id: NodeId) -> Result<Option<NodeEntity>> {
        select_node(&self.db_pool, id).context("Failed to select node")
    }

    fn update_node(&self, node: &NodeEntity) -> Result<NodeEntity> {
        update_node(&self.db_pool, node, None)
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
            let children = get_all_children(db_pool, node_id).context("Failed to get children")?;

            for child in children {
                delete_recursively(db_pool, child.id, deleted_nodes)?;
            }

            let node = select_node(db_pool, node_id)
                .context("Failed to select node")?
                .context("Node not found")?;

            if let Some(parent_id) = node.parent_id {
                let parent_node = select_node(db_pool, parent_id)
                    .context("Failed to select parent")?
                    .context("Parent not found")?;

                let deleted_node = delete_node(db_pool, node_id, &parent_node.metadata)
                    .context("Failed to delete node")?;
                deleted_nodes.push(deleted_node);
            } else {
                let empty_metadata = node.metadata.clone();
                let deleted_node = delete_node(db_pool, node_id, &empty_metadata)
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
        operations::move_node(&self.db_pool, id, from, from_metadata, to, to_metadata)
    }

    fn get_children(&self, parent_id: NodeId) -> Result<Vec<NodeEntity>> {
        get_all_children(&self.db_pool, parent_id).context("Failed to get children")
    }

    fn move_node_to_trash(
        &self,
        id: NodeId,
        from: NodeId,
        from_metadata: EncryptedMetadata,
        to_trash: NodeId,
        to_trash_metadata: EncryptedMetadata,
    ) -> Result<()> {
        let mut conn = self
            .db_pool
            .get()
            .context("Failed to get database connection")?;

        conn.transaction(|conn| {
            use crate::db::schema::nodes::dsl as NodeDsl;
            let now = chrono::Utc::now().naive_utc();

            diesel::update(NodeDsl::Node)
                .filter(NodeDsl::id.eq(from))
                .set((
                    NodeDsl::metadata.eq(&from_metadata),
                    NodeDsl::metadata_change_counter.eq(NodeDsl::metadata_change_counter + 1),
                ))
                .execute(conn)
                .context("Failed to update old parent")?;

            diesel::update(NodeDsl::Node)
                .filter(NodeDsl::id.eq(to_trash))
                .set((
                    NodeDsl::metadata.eq(&to_trash_metadata),
                    NodeDsl::metadata_change_counter.eq(NodeDsl::metadata_change_counter + 1),
                ))
                .execute(conn)
                .context("Failed to update trash parent")?;

            diesel::update(NodeDsl::Node)
                .filter(NodeDsl::id.eq(id))
                .set((
                    NodeDsl::parent_id.eq(Some(to_trash)),
                    NodeDsl::deleted_on.eq(Some(now)),
                    NodeDsl::metadata_change_counter.eq(NodeDsl::metadata_change_counter + 1),
                ))
                .execute(conn)
                .context("Failed to move node to trash")?;

            Ok::<(), anyhow::Error>(())
        })?;

        Ok(())
    }

    fn move_node_out_of_trash(
        &self,
        id: NodeId,
        from_trash: NodeId,
        from_trash_metadata: EncryptedMetadata,
        to: NodeId,
        to_metadata: EncryptedMetadata,
    ) -> Result<()> {
        let mut conn = self
            .db_pool
            .get()
            .context("Failed to get database connection")?;

        conn.transaction(|conn| {
            use crate::db::schema::nodes::dsl as NodeDsl;

            diesel::update(NodeDsl::Node)
                .filter(NodeDsl::id.eq(from_trash))
                .set((
                    NodeDsl::metadata.eq(&from_trash_metadata),
                    NodeDsl::metadata_change_counter.eq(NodeDsl::metadata_change_counter + 1),
                ))
                .execute(conn)
                .context("Failed to update trash parent")?;

            diesel::update(NodeDsl::Node)
                .filter(NodeDsl::id.eq(to))
                .set((
                    NodeDsl::metadata.eq(&to_metadata),
                    NodeDsl::metadata_change_counter.eq(NodeDsl::metadata_change_counter + 1),
                ))
                .execute(conn)
                .context("Failed to update new parent")?;

            diesel::update(NodeDsl::Node)
                .filter(NodeDsl::id.eq(id))
                .set((
                    NodeDsl::parent_id.eq(Some(to)),
                    NodeDsl::deleted_on.eq(None::<NaiveDateTime>),
                    NodeDsl::metadata_change_counter.eq(NodeDsl::metadata_change_counter + 1),
                ))
                .execute(conn)
                .context("Failed to move node out of trash")?;

            Ok::<(), anyhow::Error>(())
        })?;

        Ok(())
    }
}
