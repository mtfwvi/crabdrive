use crate::utils::browser::{LocalStorage, SessionStorage, redirect};
use crate::{api, utils};

use crabdrive_common::payloads::auth::request::login::PostLoginRequest;
use crabdrive_common::payloads::auth::response::login::PostLoginResponse;

use anyhow::{Context, Result, anyhow};
use tracing::{debug, debug_span};

/// Attempts to authenticate a user, with username and (unencrypted) password.
///
/// This function does not return, if successful. Instead, it redirects to the URL.
pub async fn login(username: &str, password: &str, remember_username: bool) -> Result<()> {
    let _guard = debug_span!("api::login").entered();

    if utils::auth::is_authenticated()? {
        tracing::error!("Cannot sign in while already signed in. Sign out first.");
        return Err(anyhow!("Please sign out first and try again"));
    }

    let local_redirect_url = LocalStorage::get::<String>("redirect_url")?;

    SessionStorage::clear().context("Failed to clear SessionStore")?;

    let salt = utils::auth::salt_from_username(username).await;
    let (server_password, derived_key) =
        utils::encryption::auth::derive_from_password(password, &salt)?;

    debug!("Attempting to login");

    let response = api::requests::auth::post_login(PostLoginRequest {
        username: username.parse()?,
        password: server_password,
    })
    .await
    .context("Server currently not reachable - Please try again later")?;

    let login_response = match response {
        PostLoginResponse::Ok(login_success) => Ok(login_success),
        PostLoginResponse::Unauthorized(_) => {
            debug!("Login denied (Invalid Username or Password)");
            Err(anyhow!("Invalid credentials"))
        }
    }?;

    if login_response.user_keys.is_none() || login_response.should_initialize_encryption {
        tracing::error!(
            "Encryption uninitialized! Crabdrive does currently not support on-the-fly encryption initialization."
        );
        return Err(anyhow!("Encryption uninitialized."));
    }

    SessionStorage::set("bearer", &login_response.bearer_token)
        .context("Failed to persist login information.")?;

    let keys = login_response.user_keys.unwrap();

    let master_key = utils::encryption::unwrap_key(&keys.master_key, &derived_key)
        .await
        .inspect_err(|_| tracing::error!("Failed to unwrap master key"))?;
    let root_key = utils::encryption::unwrap_key(&keys.root_key, &master_key)
        .await
        .inspect_err(|_| tracing::error!("Failed to unwrap root key"))?;
    let trash_key = utils::encryption::unwrap_key(&keys.trash_key, &master_key)
        .await
        .inspect_err(|_| tracing::error!("Failed to unwrap trash key"))?;

    // TODO: Unwrap assymmetrical keys

    super::fetch_user_nodes(
        login_response.root_node_id,
        &root_key,
        login_response.trash_node_id,
        &trash_key,
    )
    .await?;

    // Store keys in session storage
    // See also: https://github.com/mtfwvi/crabdrive/issues/114
    SessionStorage::set("master_key", &utils::encryption::encode_key(&master_key))?;
    SessionStorage::set("root_key", &utils::encryption::encode_key(&root_key))?;
    SessionStorage::set("trash_key", &utils::encryption::encode_key(&trash_key))?;

    // Store username in storage
    SessionStorage::set("username", &username)?;

    if remember_username {
        // Store last username in local storage to remember
        LocalStorage::set("last_user", &username)?;
    } else {
        let _ = LocalStorage::remove("last_user");
    }

    // Store Root ID + Trash ID in session storage
    SessionStorage::set("root_id", &login_response.root_node_id)?;
    SessionStorage::set("trash_id", &login_response.trash_node_id)?;

    if let Some(local_redirect_url) = local_redirect_url {
        redirect(&local_redirect_url, false)?;
    } else {
        redirect(&login_response.redirect_url, true).inspect_err(|_| {
            // Clear session storage on error
            SessionStorage::clear().expect("Failed to clear session storage!");
        })?;
    }

    Ok(())
}
