use crate::model::node::DecryptedNode;
use crate::utils::browser::SessionStorage;
use crate::{api, utils};

use crabdrive_common::payloads::node::response::node::GetNodeResponse;
use crabdrive_common::uuid::UUID;

use anyhow::{Context, Result, anyhow};
use tracing::debug_span;

/// Get the trash node of the currently authenticated user. Returns `Err` if called unauthenticated.
pub async fn get_trash_node() -> Result<DecryptedNode> {
    let _guard = debug_span!("api::getTrashNode").entered();
    let token = utils::auth::get_token()
        .inspect_err(|_| tracing::error!("No token found. Is the user authenticated?"))?;

    let trash_id: UUID = SessionStorage::get("trash_id")
        // Currently there is no way to retrieve the root node / trash node ID, and they are only
        // transmitted during login.
        .context("Failed to retrieve trash node. Please check if local storage is enabled and re-authenticate")?
        // The user is an idiot and cleared session storage
        .ok_or(anyhow!("Trash node not found. Please stop tampering with session storage and re-authenticate"))?;

    let get_node_response = api::requests::node::get_node(trash_id, &token)
        .await
        .inspect_err(|e| tracing::error!("Failed to get trash node: {}", e))?;

    match get_node_response {
        GetNodeResponse::Ok(encrypted_node) => {
            let decrypted_node = utils::encryption::node::decrypt_node(
                encrypted_node,
                utils::encryption::auth::get_trash_key()
                    .inspect_err(|_| tracing::error!("Failed to get trash encryption key."))?,
            )
            .await
            .inspect_err(|e| tracing::error!("Failed to get decrypt trash node: {}", e))
            .context("Failed to decrypt trash node")?;

            Ok(decrypted_node)
        }
        GetNodeResponse::NotFound => Err(anyhow!("No trash node found. Please re-authenticate")),
    }
}
