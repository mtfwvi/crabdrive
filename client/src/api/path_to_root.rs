use crate::api::get_accessible_path;
use crate::model::node::DecryptedNode;
use anyhow::Result;
use crabdrive_common::storage::NodeId;
use tracing::debug_span;

//TODO should be removed
pub async fn path_to_root(from_node_id: NodeId) -> Result<Vec<DecryptedNode>> {
    let _guard = debug_span!("api::pathToRoot").entered();

    get_accessible_path(from_node_id)
        .await
        .inspect_err(|e| tracing::error!("Failed to get path between node and root node: {}", e))
}
