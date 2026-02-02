use crate::api::requests::node::get_path_between_nodes;
use crate::model::node::DecryptedNode;
use crate::utils::encryption::node::decrypt_node_with_parent;
use crabdrive_common::payloads::node::response::node::GetPathBetweenNodesResponse;
use crabdrive_common::storage::NodeId;
use anyhow::{anyhow, Result};

pub async fn path_between_nodes(
    from_node: DecryptedNode,
    to_node_id: NodeId,
) -> Result<Vec<DecryptedNode>> {
    let path_response = get_path_between_nodes(from_node.id, to_node_id, &"".to_string()).await?;
    match path_response {
        GetPathBetweenNodesResponse::Ok(encrypted_nodes) => {
            // we need to decrypt all nodes with their parent
            let mut decrypted_nodes: Vec<DecryptedNode> = Vec::with_capacity(encrypted_nodes.len());
            decrypted_nodes.push(from_node);

            for encrypted_node in encrypted_nodes.iter().skip(1) {
                let decryption_result = decrypt_node_with_parent(
                    // last cannot be None, as the vec contains from node
                    decrypted_nodes.last().unwrap(),
                    encrypted_node.clone(),
                )
                .await;

                decrypted_nodes.push(decryption_result?);
            }

            Ok(decrypted_nodes)
        }
        GetPathBetweenNodesResponse::NoContent => {
            Err(anyhow!("the path between the nodes does not exist"))
        }
        GetPathBetweenNodesResponse::NotFound => Err(anyhow!("one of the nodes does not exist")),
    }
}
