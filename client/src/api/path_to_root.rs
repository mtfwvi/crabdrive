use crate::api::{get_root_node, path_between_nodes};
use crate::model::node::DecryptedNode;
use anyhow::{Context, Result};
use crabdrive_common::storage::NodeId;

pub async fn path_to_root(from_node_id: NodeId) -> Result<Vec<DecryptedNode>> {
    let root_node = get_root_node().await.context("Failed to get root node")?;

    path_between_nodes(root_node, from_node_id).await
}
