use crate::api::requests::node::get_path_between_nodes;
use crate::model::node::DecryptedNode;
use crate::utils;
use crate::utils::encryption::node::decrypt_node_with_parent;
use anyhow::{Result, anyhow};
use crabdrive_common::payloads::node::response::node::GetPathBetweenNodesResponse;
use crabdrive_common::storage::NodeId;
use tracing::debug_span;

pub async fn path_between_nodes(
    from_node: DecryptedNode,
    to_node_id: NodeId,
) -> Result<Vec<DecryptedNode>> {
    let _guard = debug_span!("api::pathBetweenNodes").entered();
    let token = utils::auth::get_token()
        .inspect_err(|_| tracing::error!("No token found. Is the user authenticated?"))?;

    let path_response = get_path_between_nodes(from_node.id, to_node_id, &token)
        .await
        .inspect_err(|e| tracing::error!("Failed to get path between nodes: {}", e))?;

    match path_response {
        GetPathBetweenNodesResponse::Ok(encrypted_nodes) => {
            // we need to decrypt all nodes with their parent
            let mut decrypted_nodes: Vec<DecryptedNode> = Vec::with_capacity(encrypted_nodes.len());
            decrypted_nodes.push(from_node);

            for encrypted_node in encrypted_nodes.iter().skip(1) {
                let decrypted_node = decrypt_node_with_parent(
                    // last cannot be None, as the vec contains from node
                    decrypted_nodes.last().unwrap(),
                    encrypted_node.clone(),
                )
                .await
                .inspect_err(|e| tracing::error!("Failed to decrypt node with parent: {}", e))?;

                decrypted_nodes.push(decrypted_node);
            }

            Ok(decrypted_nodes)
        }
        GetPathBetweenNodesResponse::NoContent => Err(anyhow!("Path between nodes does not exist")),
        GetPathBetweenNodesResponse::NotFound => Err(anyhow!("One of the nodes does not exist")),
    }
}
