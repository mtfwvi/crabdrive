use crate::db::NodeDsl;
use crate::db::connection::DbPool;
use crate::db::operations::node::{
    delete_node, get_all_children, get_path_between_nodes, insert_node, move_node, select_node,
    update_node,
};
use crate::db::operations::share::{get_access_list_parent_tree, has_access};
use crate::storage::node::persistence::model::node_entity::NodeEntity;
use crate::storage::revision::persistence::model::revision_entity::RevisionEntity;
use anyhow::{Context, Ok, Result};
use chrono::NaiveDateTime;
use crabdrive_common::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::storage::{NodeId, NodeType};
use crabdrive_common::user::UserId;
use diesel::Connection;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use std::sync::Arc;

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
    ///
    /// Constraints:
    /// - the node cannot be moved into one of its own children
    /// - the to_node must be a folder
    /// - a file cannot be moved into another file
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

    /// Get the node entities of the path between two nodes
    fn get_path_between_nodes(&self, from: NodeId, to: NodeId) -> Result<Option<Vec<NodeEntity>>>;

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

    fn purge_tree_from_trash(&self, id: NodeId) -> Result<(Vec<NodeEntity>, Vec<RevisionEntity>)>;

    fn purge_nodes_from_trash(
        &self,
        node_ids: Vec<NodeId>,
        trash_node_id: NodeId,
        new_trash_metadata: EncryptedMetadata,
    ) -> Result<(Vec<NodeEntity>, Vec<RevisionEntity>)>;

    /// Get the path from a node to the root or trash node
    fn get_path_to_root(&self, node: NodeId) -> Result<Vec<NodeEntity>>;

    /// Check if a user has access to the node (own nodes & shared nodes)
    fn has_access(&self, id: NodeId, user: UserId) -> Result<bool>;

    /// Get a list of tuples `(UserId, Username)`, on which users have access to a node
    fn get_access_list(&self, node: NodeId) -> Result<Vec<(UserId, String)>>;
}

pub struct NodeRepositoryImpl {
    db_pool: Arc<DbPool>,
}

impl NodeRepositoryImpl {
    pub fn new(db_pool: Arc<DbPool>) -> Self {
        Self { db_pool }
    }
}

impl NodeRepository for NodeRepositoryImpl {
    fn create_node(
        &self,
        parent: Option<NodeId>,
        encrypted_metadata: EncryptedMetadata,
        owner: UserId,
        node_type: NodeType,
        node_id: NodeId,
    ) -> Result<NodeEntity> {
        let mut conn = self.db_pool.get().context("Failed to get db connection")?;

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
        let mut conn = self.db_pool.get().context("Failed to get db connection")?;
        select_node(&mut conn, id).context("Failed to select node")
    }

    fn update_node(&self, node: &NodeEntity) -> Result<NodeEntity> {
        let mut conn = self.db_pool.get().context("Failed to get db connection")?;
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
            let mut conn = db_pool.get().context("Failed to get db connection")?;

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
        let mut conn = self.db_pool.get().context("Failed to get db connection")?;

        // to_node must be a folder
        let to_node = select_node(&mut conn, to)
            .context("Failed to select to_node")?
            .context("to_node not found")?;

        if to_node.node_type != NodeType::Folder {
            anyhow::bail!("Cannot move a node into a non-folder node");
        }

        // node cannot be moved into one of its own children (i.e. to_node cannot be in the subtree of id)
        let path = get_path_between_nodes(&mut conn, id, to)?;
        if !path.is_empty() && path.first().map(|n| n.id) == Some(id) {
            anyhow::bail!("Cannot move a node into one of its own children");
        }

        move_node(&mut conn, id, from, from_metadata, to, to_metadata)
    }

    fn get_children(&self, parent_id: NodeId) -> Result<Vec<NodeEntity>> {
        let mut conn = self.db_pool.get().context("Failed to get db connection")?;
        get_all_children(&mut conn, parent_id).context("Failed to get children")
    }

    fn get_path_between_nodes(&self, from: NodeId, to: NodeId) -> Result<Option<Vec<NodeEntity>>> {
        let mut conn = self.db_pool.get().context("Failed to get db connection")?;
        let path: Vec<NodeEntity> = get_path_between_nodes(&mut conn, from, to)?;

        if path[0].id != from {
            Ok(None)
        } else {
            Ok(Some(path))
        }
    }

    fn move_node_to_trash(
        &self,
        id: NodeId,
        from: NodeId,
        from_metadata: EncryptedMetadata,
        to_trash: NodeId,
        to_trash_metadata: EncryptedMetadata,
    ) -> Result<()> {
        self.move_node(id, from, from_metadata, to_trash, to_trash_metadata)?;

        let mut conn = self
            .db_pool
            .get()
            .context("Failed to get database connection")?;

        conn.transaction(|conn| {
            let now = chrono::Local::now().naive_local();

            diesel::update(NodeDsl::Node)
                .filter(NodeDsl::id.eq(id))
                .set(NodeDsl::deleted_on.eq(Some(now)))
                .execute(conn)
                .context("Failed to set deleted_on timestamp")?;

            Ok(())
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
        self.move_node(id, from_trash, from_trash_metadata, to, to_metadata)?;

        let mut conn = self
            .db_pool
            .get()
            .context("Failed to get database connection")?;

        conn.transaction(|conn| {
            diesel::update(NodeDsl::Node)
                .filter(NodeDsl::id.eq(id))
                .set(NodeDsl::deleted_on.eq(None::<NaiveDateTime>))
                .execute(conn)
                .context("Failed to clear deleted_on timestamp")?;

            Ok(())
        })?;

        Ok(())
    }

    fn purge_tree_from_trash(&self, id: NodeId) -> Result<(Vec<NodeEntity>, Vec<RevisionEntity>)> {
        use crate::db::{NodeDsl, RevisionDsl};

        let mut conn = self
            .db_pool
            .get()
            .context("Failed to get database connection")?;

        conn.transaction(|conn| {
            let mut all_nodes = Vec::new();
            let mut all_revisions = Vec::new();

            fn collect_tree_nodes(
                conn: &mut diesel::SqliteConnection,
                node_id: NodeId,
                nodes: &mut Vec<NodeEntity>,
            ) -> Result<()> {
                let node = NodeDsl::Node
                    .filter(NodeDsl::id.eq(node_id))
                    .first::<NodeEntity>(conn)
                    .context("Node not found")?;

                if node.deleted_on.is_none() {
                    anyhow::bail!("Cannot purge node tree that is not in trash");
                }

                let children = NodeDsl::Node
                    .filter(NodeDsl::parent_id.eq(node_id))
                    .load::<NodeEntity>(conn)
                    .context("Failed to load children")?;

                for child in children {
                    collect_tree_nodes(conn, child.id, nodes)?;
                }

                nodes.push(node);

                Ok(())
            }

            collect_tree_nodes(conn, id, &mut all_nodes)?;

            for node in &all_nodes {
                let revisions = RevisionDsl::Revision
                    .filter(RevisionDsl::file_id.eq(node.id))
                    .load::<RevisionEntity>(conn)
                    .context("Failed to load revisions")?;

                all_revisions.extend(revisions);

                diesel::delete(RevisionDsl::Revision)
                    .filter(RevisionDsl::file_id.eq(node.id))
                    .execute(conn)
                    .context("Failed to delete revisions")?;

                diesel::delete(NodeDsl::Node)
                    .filter(NodeDsl::id.eq(node.id))
                    .execute(conn)
                    .context("Failed to delete node")?;
            }

            Ok((all_nodes, all_revisions))
        })
    }

    fn purge_nodes_from_trash(
        &self,
        node_ids: Vec<NodeId>,
        trash_node_id: NodeId,
        new_trash_metadata: EncryptedMetadata,
    ) -> Result<(Vec<NodeEntity>, Vec<RevisionEntity>)> {
        let mut all_nodes = Vec::new();
        let mut all_revisions = Vec::new();

        for node_id in node_ids {
            let (nodes, revisions) = self.purge_tree_from_trash(node_id)?;
            all_nodes.extend(nodes);
            all_revisions.extend(revisions);
        }

        let mut conn = self
            .db_pool
            .get()
            .context("Failed to get database connection")?;

        diesel::update(NodeDsl::Node)
            .filter(NodeDsl::id.eq(trash_node_id))
            .set(NodeDsl::metadata.eq(new_trash_metadata))
            .execute(&mut conn)
            .context("Failed to update trash node metadata")?;

        Ok((all_nodes, all_revisions))
    }

    fn get_path_to_root(&self, node: NodeId) -> Result<Vec<NodeEntity>> {
        let mut conn = self.db_pool.get().context("Failed to get db connection")?;
        get_path_between_nodes(&mut conn, NodeId::nil(), node)
    }

    fn has_access(&self, id: NodeId, user: UserId) -> Result<bool> {
        let mut conn = self.db_pool.get().context("Failed to get db connection")?;
        has_access(&mut conn, id, user)
    }

    fn get_access_list(&self, node: NodeId) -> Result<Vec<(UserId, String)>> {
        let mut conn = self.db_pool.get().context("Failed to get db connection")?;
        get_access_list_parent_tree(&mut conn, node)
    }
}
