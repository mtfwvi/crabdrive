mod login;
mod logout;
mod register;

pub use login::login;
pub use logout::logout;
pub use register::register;

use crate::model::encryption::MetadataKey;
use crate::model::node::NodeMetadata;
use crate::utils::browser::SessionStorage;
use crate::{api, utils};

use crabdrive_common::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::payloads::node::request::node::PatchNodeRequest;
use crabdrive_common::payloads::node::response::node::GetNodeResponse;
use crabdrive_common::storage::NodeId;

use anyhow::Result;
use tracing::debug_span;
#[cfg(debug_assertions)]
use wasm_bindgen::prelude::wasm_bindgen;

// The next three functions are used for testing via browser console. They can be accessed via:
// - await window.wasmBindings._register_user()
// - await window.wasmBindings._login_user()
// - await window.wasmBindings._logout_user()
// Only present when building in debug mode.

#[cfg(debug_assertions)]
#[wasm_bindgen]
pub async fn _login_user(username: &str, password: &str) {
    let result = login(username, password, false).await;
    if result.is_err() {
        tracing::error!("Failed to login: {:?}", result);
    }
}

#[cfg(debug_assertions)]
#[wasm_bindgen]
pub async fn _register_user(username: &str, password: &str, invite_code: &str) {
    let result = register(username, password, invite_code).await;
    if result.is_err() {
        tracing::error!("Failed to register: {:?}", result);
    }
}

#[cfg(debug_assertions)]
#[wasm_bindgen]
pub async fn _logout_user() {
    let result = logout().await;
    if result.is_err() {
        tracing::error!("Failed to logout: {:?}", result);
    }
}

/// Updates the cached node IDs, and initializes (if uninitialized)
async fn fetch_user_nodes(
    root_node_id: NodeId,
    root_key: &MetadataKey,
    trash_node_id: NodeId,
    trash_key: &MetadataKey,
) -> Result<()> {
    let _guard = debug_span!("fetchUserNodes").entered();
    // Check if root node or trash node contains NIL metadata. If so, both are uninitialized
    // and need to be initialized first.
    let root_node_reponse = match api::requests::node::get_node(root_node_id).await? {
        GetNodeResponse::Ok(node) => Ok(node),
        GetNodeResponse::NotFound => Err(anyhow::anyhow!("Root node not found.")),
    }?;

    let trash_node_response = match api::requests::node::get_node(trash_node_id).await? {
        GetNodeResponse::Ok(node) => Ok(node),
        GetNodeResponse::NotFound => Err(anyhow::anyhow!("Trash node not found.")),
    }?;

    if root_node_reponse.encrypted_metadata == EncryptedMetadata::nil() {
        tracing::debug!("Root node uninitialized. Initializing.");
        // Root node uninitialized
        api::requests::node::patch_node(
            root_node_id,
            PatchNodeRequest {
                node_change_count: root_node_reponse.change_count,
                node_metadata: utils::encryption::node::encrypt_metadata(
                    &NodeMetadata::v1(
                        "Root".to_string(),
                        chrono::Local::now().naive_local(),
                        chrono::Local::now().naive_local(),
                        None,
                        None,
                        None,
                        Vec::new(),
                    ),
                    root_key,
                )
                .await?,
            },
        )
        .await?;
    }

    if trash_node_response.encrypted_metadata == EncryptedMetadata::nil() {
        tracing::debug!("Trash node uninitialized. Initializing..");
        // Trash node uninitialized
        api::requests::node::patch_node(
            trash_node_id,
            PatchNodeRequest {
                node_change_count: root_node_reponse.change_count,
                node_metadata: utils::encryption::node::encrypt_metadata(
                    &NodeMetadata::v1(
                        "Trash".to_string(),
                        chrono::Local::now().naive_local(),
                        chrono::Local::now().naive_local(),
                        None,
                        None,
                        None,
                        Vec::new(),
                    ),
                    trash_key,
                )
                .await?,
            },
        )
        .await?;
    }

    // Store Root ID + Trash ID in session storage
    SessionStorage::set("root_id", &root_node_id)?;
    SessionStorage::set("trash_id", &trash_node_id)?;

    Ok(())
}
