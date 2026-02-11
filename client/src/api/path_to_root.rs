use crate::api::{get_root_node, path_between_nodes};
use crate::model::node::DecryptedNode;
use anyhow::Result;
use crabdrive_common::storage::NodeId;
use tracing::debug_span;

pub async fn path_to_root(from_node_id: NodeId) -> Result<Vec<DecryptedNode>> {
    let _guard = debug_span!("api::pathToRoot").entered();
    let root_node = get_root_node()
        .await
        .inspect_err(|e| tracing::error!("Failed to get root node: {}", e))?;
    path_between_nodes(root_node, from_node_id)
        .await
        .inspect_err(|e| tracing::error!("Failed to get path between node and root node: {}", e))
}
