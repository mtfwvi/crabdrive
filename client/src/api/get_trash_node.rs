use crate::model::node::DecryptedNode;
use crate::utils::browser::SessionStorage;
use crate::{api, utils};

use crabdrive_common::payloads::node::response::node::GetNodeResponse;
use crabdrive_common::uuid::UUID;

use anyhow::{Context, Result, anyhow};

/// Get the trash node of the currently authenticated user. Returns `Err` if called unauthenticated.
pub async fn get_trash_node() -> Result<DecryptedNode> {
    let token = utils::auth::get_token()?;
    let trash_id: UUID = SessionStorage::get("trash_id")
        // Currently there is no way to retrieve the root node / trash node ID, and they are only
        // transmitted during login.
        .context("Failed to retrieve root node. Please check if local storage is enabled and re-authenticate")?
        // The user is an idiot and cleared session storage
        .ok_or(anyhow!("Root node not found. Please stop tampering with session storage and re-authenticate"))?;

    let get_node_response = api::requests::node::get_node(trash_id, &token).await?;

    match get_node_response {
        GetNodeResponse::Ok(encrypted_node) => {
            let decrypted_node_result = utils::encryption::node::decrypt_node(
                encrypted_node,
                utils::encryption::auth::get_root_key()?,
            )
            .await;

            if let Err(js_error) = decrypted_node_result {
                return Err(anyhow!("could not decrypt node: {:?}", js_error));
            }

            let decrypted_node = decrypted_node_result?;
            Ok(decrypted_node)
        }
        GetNodeResponse::NotFound => Err(anyhow!("No trash node found. Please re-authenticate")),
    }
}
