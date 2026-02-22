use crate::api::requests;
use crate::api::requests::node::get_node;
use crate::model::node::DecryptedNode;
use crate::utils::auth::get_token;
use crate::utils::encryption::auth::get_master_key;
use crate::utils::encryption::node::{decrypt_node, decrypt_node_path};
use anyhow::{Context, Result, anyhow};
use crabdrive_common::payloads::node::response::node::{
    GetAccessiblePathResponse, GetNodeResponse,
};
use crabdrive_common::storage::NodeId;

// returns the path from a shared node to the topmost node the user has access to
pub async fn get_accessible_path(node_id: NodeId) -> Result<Vec<DecryptedNode>> {
    let token = get_token()?;

    let response = requests::node::get_accessible_path(node_id, &token)
        .await
        .context("Failed to get path to shared node")?;

    let encrypted_path = match response {
        GetAccessiblePathResponse::Ok(encrypted_path) => encrypted_path,
        GetAccessiblePathResponse::NotFound => {
            return Err(anyhow!(
                "Server returned 404 when getting path to shared node"
            ));
        }
    };

    let first_node_id = encrypted_path
        .first()
        .ok_or(anyhow!("path returned by server is empty"))?;

    // assume the first node in the path is a shared node that the user has access to
    // TODO this could be used for all nodes by checking whether the first node id is the root or shared node which could make the ui simpler
    let node_response = get_node(first_node_id.id, &token).await?;
    let decrypted_first_node = match node_response {
        GetNodeResponse::Ok(node) => {
            let master_key = get_master_key()?;
            decrypt_node(node, master_key).await?
        }
        GetNodeResponse::NotFound => {
            return Err(anyhow!("Node not found"));
        }
    };

    decrypt_node_path(decrypted_first_node, encrypted_path).await
}
