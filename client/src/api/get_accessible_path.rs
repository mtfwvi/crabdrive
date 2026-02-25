use crate::api::{get_shared_node_encryption_key, requests};
use crate::model::node::DecryptedNode;
use crate::utils::browser::SessionStorage;
use crate::utils::encryption::auth::{get_master_key, get_root_key};
use crate::utils::encryption::node::{decrypt_node, decrypt_node_path};
use crate::utils::encryption::unwrap_key;
use anyhow::{Context, Result, anyhow};
use crabdrive_common::payloads::node::response::node::GetAccessiblePathResponse;
use crabdrive_common::storage::NodeId;

// returns the path from a shared node to the topmost node the user has access to
pub async fn get_accessible_path(node_id: NodeId) -> Result<Vec<DecryptedNode>> {
    let response = requests::node::get_accessible_path(node_id)
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
        let wrapped_encryption_key = get_shared_node_encryption_key(first_node.id).await?;
        let master_key = get_master_key()?;
        let unwrapped_key = unwrap_key(&wrapped_encryption_key, &master_key).await?;

        decrypt_node(first_node.clone(), unwrapped_key).await?
    };

    decrypt_node_path(decrypted_first_node, encrypted_path).await
}
