use crate::api::requests::share::get_accepted_shared_nodes;
use crate::model::node::DecryptedNode;
use crate::utils::encryption::auth::get_master_key;
use crate::utils::encryption::node::decrypt_node;
use crate::utils::encryption::unwrap_key;
use anyhow::Result;
use crabdrive_common::payloads::node::response::share::GetAcceptedSharedResponse;

pub async fn get_accepted_nodes() -> Result<Vec<DecryptedNode>> {
    let response = get_accepted_shared_nodes().await?;

    let GetAcceptedSharedResponse::Ok(encrypted_node_with_keys) = response;
    let master_key = get_master_key()?;

    let mut decrypted_nodes = Vec::with_capacity(encrypted_node_with_keys.len());

    for (wrapped_encryption_key, encrypted_node) in encrypted_node_with_keys {
        let unwrapped_key = unwrap_key(&wrapped_encryption_key, &master_key).await?;

        let decrypted_node = decrypt_node(encrypted_node, unwrapped_key).await?;
        decrypted_nodes.push(decrypted_node);
    }

    Ok(decrypted_nodes)
}