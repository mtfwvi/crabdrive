use crate::utils::browser::SessionStorage;
use crate::{api, utils};

use anyhow::{Context, Result, anyhow};
use tracing::debug_span;

/// Logs a user out
pub async fn logout() -> Result<()> {
    let _guard = debug_span!("api::logout").entered();

    if !utils::auth::is_authenticated()? {
        tracing::error!("Cannot sign out, because you are not signed in.");
        return Err(anyhow!("Unable to sign out at the moment"));
    }

    api::requests::auth::post_logout()
        .await
        .context("Server currently not reachable - Please try again later")?;

    SessionStorage::clear().context("Failed to clear session storage!")?;

    Ok(())
}
