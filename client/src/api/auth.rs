use crate::model::node::NodeMetadata;
use crate::utils::browser::{LocalStorage, SessionStorage};
use crate::{api, utils};

use crabdrive_common::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::encryption_key::EncryptionKey;
use crabdrive_common::payloads::auth::request::register::PostRegisterRequest;
use crabdrive_common::payloads::auth::response::login::UserKeys;
use crabdrive_common::payloads::auth::response::register::PostRegisterResponse;
use crabdrive_common::payloads::node::request::node::PatchNodeRequest;
use crabdrive_common::payloads::node::response::node::GetNodeResponse;

use crate::model::encryption::MetadataKey;
use anyhow::{Context, Result, anyhow};
use crabdrive_common::payloads::auth::{
    request::login::PostLoginRequest, response::login::PostLoginResponse,
};
use crabdrive_common::storage::NodeId;
use tracing::{debug, debug_span};

/// Checks if a username is remembered
pub async fn get_remembered_username() -> Result<Option<String>> {
    LocalStorage::get::<String>("last_user")
}

/// Attempts to register a user, with a username and (unencrypted) password.
pub async fn register(username: String, password: String) -> Result<()> {
    if !utils::auth::password_is_secure(&password) {
        return Err(anyhow!("Password does not meet minimum requirements!"));
    }

    let _guard = debug_span!("api::register", username = username, password = password).entered();

    let salt = utils::auth::salt_from_username(&username);
    let hash = utils::auth::hash_password(&password, &salt);
    debug!("Hash: {}:{}", &username, &hash);

    // Generate a new master key, root node & trash node keys
    let master_key = utils::encryption::generate_aes256_key().await?;
    let root_key = utils::encryption::generate_aes256_key().await?;
    let trash_key = utils::encryption::generate_aes256_key().await?;

    let derived_key = utils::encryption::auth::derive_from_password(&hash, &salt).await?;

    let wrapped_master_key = utils::encryption::wrap_key(master_key, &derived_key).await?;
    let wrapped_root_key = utils::encryption::wrap_key(root_key, &derived_key).await?;
    let wrapped_trash_key = utils::encryption::wrap_key(trash_key, &derived_key).await?;

    let response = api::requests::auth::post_register(PostRegisterRequest {
        username: username.clone(),
        password: hash.clone(),
        keys: UserKeys::new(
            vec![],
            EncryptionKey::nil(),
            wrapped_master_key,
            wrapped_root_key,
            wrapped_trash_key,
        ),
    })
    .await
    .context("Server currently not reachable - Please try again later")?;

    match response {
        PostRegisterResponse::Created => Ok(()),
        PostRegisterResponse::Unauthorized => {
            Err(anyhow!("You are unauthorized to create a new account."))
        }
        PostRegisterResponse::Conflict(reason) => {
            Err(anyhow!("Failed to create a new account: {}", reason))
        }
    }
}

/// Attempts to authenticate a user, with username and (unencrypted) password.
///
/// This function does not return, if successful. Instead, it redirects to the URL.
pub async fn login(username: String, password: String, remeber_username: bool) -> Result<()> {
    let _guard = debug_span!("api::login", username = username, password = password).entered();

    SessionStorage::clear().context("Failed to clear SessionStore")?;

    let salt = utils::auth::salt_from_username(&username);
    let hash = utils::auth::hash_password(&password, &salt);

    debug!("Attempting to login");

    let response = api::requests::auth::post_login(PostLoginRequest {
        username: username.clone(),
        password: hash.clone(),
    })
    .await
    .context("Server currently not reachable - Please try again later")?;

    let login_response = match response {
        PostLoginResponse::Ok(login_success) => Ok(login_success),
        PostLoginResponse::Unauthorized(login_denied_reason) => {
            debug!("Login denied: {:?}", login_denied_reason);
            Err(anyhow::anyhow!("Invalid credentials"))
        }
    }?;

    if login_response.user_keys.is_none() || login_response.should_initialize_encryption {
        return Err(anyhow::anyhow!("Encryption uninitialized."));
    }

    SessionStorage::set("bearer", &login_response.bearer_token)
        .context("Failed to persist login information.")?;

    let keys = login_response.user_keys.unwrap();

    let derived_key = utils::encryption::auth::derive_from_password(&hash, &salt).await?;
    let master_key = utils::encryption::unwrap_key(keys.master_key, derived_key).await?;
    let root_key = utils::encryption::unwrap_key(keys.root_key, master_key).await?;
    let trash_key = utils::encryption::unwrap_key(keys.trash_key, master_key).await?;

    // TODO: Unwrap assymmetrical keys

    // Store keys in session storage
    // See also: https://github.com/mtfwvi/crabdrive/issues/114
    SessionStorage::set("master_key", &utils::encryption::encode_key(&master_key))?;
    SessionStorage::set("root_key", &utils::encryption::encode_key(&root_key))?;
    SessionStorage::set("trash_key", &utils::encryption::encode_key(&trash_key))?;

    if remeber_username {
        LocalStorage::set("last_user", &username)?;
    } else {
        let _ = LocalStorage::remove("last_user");
    }

    // Store Root ID + Trash ID in session storage
    SessionStorage::set("root_id", &login_response.root_node_id)?;
    SessionStorage::set("trash_id", &login_response.trash_node_id)?;

    utils::browser::redirect(&login_response.redirect_url, true).inspect_err(|_| {
        // Clear session storage on error
        SessionStorage::clear().expect("Failed to clear session storage!");
    })?;

    Ok(())
}

/// Updates the cached node IDs, and initializes (if uninitialized)
pub(super) async fn fetch_user_nodes(
    root_node_id: NodeId,
    root_key: &MetadataKey,
    trash_node_id: NodeId,
    trash_key: &MetadataKey,
) -> Result<()> {
    // Check if root node or trash node contains NIL metadata. If so, both are uninitialized
    // and need to be initialized first.
    let root_node_reponse =
        match api::requests::node::get_node(root_node_id, &"".to_string()).await? {
            GetNodeResponse::Ok(node) => Ok(node),
            GetNodeResponse::NotFound => Err(anyhow::anyhow!("Root node not found.")),
        }?;

    let trash_node_response =
        match api::requests::node::get_node(trash_node_id, &"".to_string()).await? {
            GetNodeResponse::Ok(node) => Ok(node),
            GetNodeResponse::NotFound => Err(anyhow::anyhow!("Trash node not found.")),
        }?;

    if root_node_reponse.encrypted_metadata == EncryptedMetadata::nil() {
        debug!("Root node uninitialized. Initializing.");
        // Root node uninitialized
        api::requests::node::patch_node(
            root_node_id,
            PatchNodeRequest {
                node_change_count: root_node_reponse.change_count + 1,
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
            &"".to_string(),
        )
        .await?;
    }

    if trash_node_response.encrypted_metadata == EncryptedMetadata::nil() {
        debug!("Trash node uninitialized. Initializing..");
        // Trash node uninitialized
        api::requests::node::patch_node(
            trash_node_id,
            PatchNodeRequest {
                node_change_count: root_node_reponse.change_count + 1,
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
            &"".to_string(),
        )
        .await?;
    }

    // Store Root ID + Trash ID in session storage
    SessionStorage::set("root_id", &root_node_id)?;
    SessionStorage::set("trash_id", &trash_node_id)?;

    Ok(())
}
