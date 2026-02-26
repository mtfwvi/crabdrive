use crate::api;
use crate::model::node::{DecryptedNode, NodeMetadata};
use crate::utils::encryption::node::decrypt_node;
use anyhow::{Context, Result, anyhow};
use crabdrive_common::payloads::node::response::node::GetNodeChildrenResponse;
use tracing::debug_span;

/// Get all children of a node
pub async fn get_children(parent: DecryptedNode) -> Result<Vec<DecryptedNode>> {
    let _guard = debug_span!("api::getChildren").entered();

    let response = api::requests::node::get_node_children(parent.id)
        .await
        .context("Failed to get children")
        .inspect_err(|_| tracing::error!("Failed to get children of node"))?;

    match response {
        GetNodeChildrenResponse::Ok(children) => {
            let mut decrypted_children = Vec::with_capacity(children.len());

            for child in children {
                let child_metadata_key = match &parent.metadata {
                    NodeMetadata::V1(metadata) => metadata.children_key.iter(),
                }
                .find(|(id, _)| *id == child.id)
                .ok_or(anyhow!("Failed to get child metadata key"))
                .inspect_err(|_| {
                    tracing::error!("Failed to find children key in parent metadata")
                })?;

                let decrypted_child = decrypt_node(child, child_metadata_key.1)
                    .await
                    .inspect_err(|e| tracing::error!("Failed to decrypt node metadata: {}", e))
                    .context("Could not decrypt node")?;
                decrypted_children.push(decrypted_child);
            }

            Ok(decrypted_children)
        }
        GetNodeChildrenResponse::NotFound => Err(anyhow!("Could not query children: 404")),
    }
}
