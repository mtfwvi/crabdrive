use crate::api::{get_accepted_nodes, requests};
use crate::model::node::DecryptedNode;
use crate::utils::auth::get_token;
use crate::utils::encryption::auth::{get_master_key, get_root_key};
use crate::utils::encryption::node::{decrypt_node, decrypt_node_path};
use anyhow::{Context, Result, anyhow};
use crabdrive_common::payloads::node::response::node::GetAccessiblePathResponse;
use crabdrive_common::storage::NodeId;
use crate::api::requests::share::get_accepted_shared_nodes;
use crate::utils::browser::SessionStorage;

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

    encrypted_path
        .first()
        .ok_or(anyhow!("path returned by server is empty"))?;


    let first_node = encrypted_path.first().unwrap();

    let decrypted_first_node = if first_node.id == SessionStorage::get("root_id")?.unwrap() {
        let key = get_root_key()?;
        decrypt_node(first_node.clone(), key).await?
    } else {
        // assume the first node in the path is a shared node that the user has access to as it is not the root node

        //TODO this is really inefficient and queries all the nodes although we just need the encryption key stored in the share entry
        let shared_nodes = get_accepted_nodes().await?;
        let first_node_decrypted = shared_nodes.iter().find(|node| node.id == first_node.id).ok_or(anyhow!("the user does not have access to this node"))?;

        first_node_decrypted.clone()
    };

    decrypt_node_path(decrypted_first_node, encrypted_path).await
}
