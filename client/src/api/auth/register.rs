use crate::utils::browser::LocalStorage;
use crate::{api, utils};

use crabdrive_common::encryption_key::EncryptionKey;
use crabdrive_common::payloads::auth::request::register::PostRegisterRequest;
use crabdrive_common::payloads::auth::response::login::UserKeys;
use crabdrive_common::payloads::auth::response::register::PostRegisterResponse;

use anyhow::{Context, Result, anyhow};
use tracing::debug_span;

/// Attempts to register a user, with a username and (unencrypted) password.
pub async fn register(username: &str, password: &str) -> Result<()> {
    let _guard = debug_span!("api::register").entered();

    if utils::auth::is_authenticated()? {
        tracing::error!("Cannot register while already signed in. Sign out first.");
        return Err(anyhow!("Please sign out first and try again"));
    }

    if !utils::auth::is_valid_password(password) {
        return Err(anyhow!("Password does not meet minimum requirements!"));
    }

    let salt = utils::auth::salt_from_username(username).await;
    let (server_password, derived_key) =
        utils::encryption::auth::derive_from_password(password, &salt)?;

    // Generate a new master key, root node & trash node keys
    let master_key = utils::encryption::generate_aes256_key().await?;
    let root_key = utils::encryption::generate_aes256_key().await?;
    let trash_key = utils::encryption::generate_aes256_key().await?;

    let wrapped_master_key = utils::encryption::wrap_key(&master_key, &derived_key)
        .await
        .inspect_err(|_| tracing::error!("Failed to wrap master key"))?;
    let wrapped_root_key = utils::encryption::wrap_key(&root_key, &master_key)
        .await
        .inspect_err(|_| tracing::error!("Failed to wrap root key"))?;
    let wrapped_trash_key = utils::encryption::wrap_key(&trash_key, &master_key)
        .await
        .inspect_err(|_| tracing::error!("Failed to wrap trash key"))?;

    let response = api::requests::auth::post_register(PostRegisterRequest {
        username: username.parse()?,
        password: server_password,
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
    }?;

    LocalStorage::set("last_user", &username)?;

    Ok(())
}
