use crate::{db::NodeDsl, storage::node::NodeEntity};

use crabdrive_common::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::storage::NodeId;

use anyhow::{Context, Result};
use diesel::{
    Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
    SqliteConnection, sql_query, sql_types::Text,
};
use tracing::instrument;

/// Get all children of a `FOLDER` node. Returns empty `Vec` for existing, but non-folder nodes.
#[instrument(skip(conn), err)]
pub fn get_all_children(conn: &mut SqliteConnection, node_id: NodeId) -> Result<Vec<NodeEntity>> {
    conn.transaction(|conn| {
        let nodes = NodeDsl::Node
            .filter(NodeDsl::parent_id.eq(node_id))
            .load::<NodeEntity>(conn)?;
        Ok(nodes)
    })
}

/// Select a node by ID
#[instrument(skip(conn), err)]
pub fn select_node(conn: &mut SqliteConnection, node_id: NodeId) -> Result<Option<NodeEntity>> {
    conn.transaction(|conn| {
        let node = NodeDsl::Node
            .filter(NodeDsl::id.eq(node_id))
            .first::<NodeEntity>(conn)
            .optional()?;
        Ok(node)
    })
}

/// Insert a new node. Updates the parent metadata, if there is a parent.
#[instrument(skip(conn), err)]
pub fn insert_node(
    conn: &mut SqliteConnection,
    node: &NodeEntity,
    parent_mdata: &EncryptedMetadata,
) -> Result<()> {
    // check that the node id is not nil as it may break the path to root function
    if node.id.eq(&NodeId::nil()) {
        return Err(anyhow::anyhow!("illegal node id"));
    }

    conn.transaction(|conn| {
        let node = diesel::insert_into(NodeDsl::Node)
            .values(node)
            .returning(NodeEntity::as_select())
            .get_result(conn)?;
        // In Parent-Node: Update metadata and increase Metadata-Counter by 1
        if node.parent_id.is_some() {
            diesel::update(NodeDsl::Node)
                .filter(NodeDsl::id.eq(node.parent_id.unwrap()))
                .set((
                    NodeDsl::metadata.eq(parent_mdata),
                    NodeDsl::metadata_change_counter.eq(NodeDsl::metadata_change_counter + 1),
                ))
                .execute(conn)?;
        }
        Ok(())
    })
}

/// Update a node and increase its metadata counter
#[instrument(skip(conn), err)]
pub fn update_node(conn: &mut SqliteConnection, node: &NodeEntity) -> Result<NodeEntity> {
    conn.transaction(|conn| {
        let node = diesel::update(NodeDsl::Node)
            .filter(NodeDsl::id.eq(node.id))
            .set((
                node,
                NodeDsl::metadata_change_counter.eq(NodeDsl::metadata_change_counter + 1),
            ))
            .returning(NodeEntity::as_select())
            .get_result(conn)?;
        Ok(node)
    })
}

// TODO: Cascade on Delete? If not, delete associated revisions and children nodes manually.
#[instrument(skip(conn), err)]
pub fn delete_node(
    conn: &mut SqliteConnection,
    node_id: NodeId,
    parent_mdata: &EncryptedMetadata,
) -> Result<NodeEntity> {
    // Delete node
    conn.transaction(|conn| {
        let node: NodeEntity = diesel::delete(NodeDsl::Node)
            .filter(NodeDsl::parent_id.is_not_null()) // Do not delete root / trash nodes
            .filter(NodeDsl::id.eq(node_id))
            .returning(NodeEntity::as_select())
            .get_result(conn)?;
        // In Parent-Node: Update metadata and increase Metadata-Counter by 1
        diesel::update(NodeDsl::Node)
            .filter(NodeDsl::id.eq(node.parent_id.unwrap()))
            .set((
                NodeDsl::metadata.eq(parent_mdata),
                NodeDsl::metadata_change_counter.eq(NodeDsl::metadata_change_counter + 1),
            ))
            .execute(conn)?;
        Ok(node)
    })
}

/// Update the parent node of a node (move the node). Requires the metadata of the old parent and
/// the metadata of the new parent.
#[instrument(skip(conn), err)]
pub fn move_node(
    conn: &mut SqliteConnection,
    id: NodeId,
    from: NodeId,
    from_metadata: EncryptedMetadata,
    to: NodeId,
    to_metadata: EncryptedMetadata,
) -> Result<()> {
    conn.transaction(|conn| {
        // Update old parent
        diesel::update(NodeDsl::Node)
            .filter(NodeDsl::id.eq(from))
            .set((
                NodeDsl::metadata.eq(&from_metadata),
                NodeDsl::metadata_change_counter.eq(NodeDsl::metadata_change_counter + 1),
            ))
            .execute(conn)
            .context("Failed to update from parent")?;
        // Update new parent
        diesel::update(NodeDsl::Node)
            .filter(NodeDsl::id.eq(to))
            .set((
                NodeDsl::metadata.eq(&to_metadata),
                NodeDsl::metadata_change_counter.eq(NodeDsl::metadata_change_counter + 1),
            ))
            .execute(conn)
            .context("Failed to update to parent")?;
        // Update the node itself
        diesel::update(NodeDsl::Node)
            .filter(NodeDsl::id.eq(id))
            .set((
                NodeDsl::parent_id.eq(Some(to)),
                NodeDsl::metadata_change_counter.eq(NodeDsl::metadata_change_counter + 1),
            ))
            .execute(conn)
            .context("Failed to move node")?;
        Ok::<(), anyhow::Error>(())
    })?;
    Ok(())
}

/// Return a list of nodes from `to_node` to `from_node` or a root node if no path exists.
///
/// **Check that the first node in the list is really the node you want and not a root node**
#[instrument(skip(conn), err)]
pub fn get_path_between_nodes(
    conn: &mut SqliteConnection,
    from: NodeId,
    to: NodeId,
) -> Result<Vec<NodeEntity>> {
    // Query that finds the path between the nodes in the tree
    // It should stop as soon as it cannot discover a new node (reached a root node) or if the desired node was reached
    let query = sql_query("\
        WITH RECURSIVE path_between_nodes AS ( \
            SELECT *,1 as _count FROM Node s1 \
            WHERE s1.id = $1 \
        UNION ALL \
            SELECT s2.*,_count+1 as _count FROM Node s2 \
            JOIN path_between_nodes s1 ON s1.parent_id = s2.id WHERE NOT s1.id = $2
        ) \
        SELECT id, parent_id, owner_id, metadata, deleted_on, metadata_change_counter, current_revision, node_type \
        FROM path_between_nodes \
        ORDER BY _count DESC \
    ").bind::<Text, _>(to.to_string()).bind::<Text, _>(from.to_string());
    Ok(query.load(conn)?)
}
