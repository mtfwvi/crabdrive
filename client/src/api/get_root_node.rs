use crate::model::node::DecryptedNode;
use crate::utils::browser::SessionStorage;
use crate::{api, utils};

use crabdrive_common::payloads::node::response::node::GetNodeResponse;
use crabdrive_common::uuid::UUID;

use anyhow::{Context, Result, anyhow};
use tracing::debug_span;

/// Get the root node of the currently authenticated user. Returns `Err` if called unauthenticated.
pub async fn get_root_node() -> Result<DecryptedNode> {
    let _guard = debug_span!("api::getRootNode").entered();
    let token = utils::auth::get_token()
        .inspect_err(|_| tracing::error!("No token found. Is the user authenticated?"))?;

    let root_id: UUID = SessionStorage::get("root_id")
        // Currently there is no way to retrieve the root node / trash node ID, and they are only
        // transmitted during login.
        .context("Failed to retrieve root node. Please check if local storage is enabled and re-authenticate")?
        // The user is an idiot and cleared session storage
        .ok_or(anyhow!("Root node not found. Please stop tampering with session storage and re-authenticate"))?;

    let get_node_response = api::requests::node::get_node(root_id, &token)
        .await
        .inspect_err(|e| tracing::error!("Failed to get root node: {}", e))?;

    match get_node_response {
        GetNodeResponse::Ok(encrypted_node) => {
            let decrypted_node = utils::encryption::node::decrypt_node(
                encrypted_node,
                utils::encryption::auth::get_root_key()
                    .inspect_err(|_| tracing::error!("Failed to get root encryption key."))?,
            )
            .await
            .inspect_err(|e| tracing::error!("Failed to get decrypt root node: {}", e))
            .context("Failed to decrypt root node")?;

            Ok(decrypted_node)
        }
        GetNodeResponse::NotFound => Err(anyhow!("No root node found. Please re-authenticate")),
    }
}
