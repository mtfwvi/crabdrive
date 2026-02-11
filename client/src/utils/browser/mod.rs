pub mod local_storage;
pub mod session_storage;

pub use local_storage::LocalStorage;
pub use session_storage::SessionStorage;

use crate::utils::error::wrap_js_err;
use anyhow::{Result, anyhow};
use web_sys::{Crypto, Document, SubtleCrypto, Url, Window};

pub fn get_window() -> Result<Window> {
    web_sys::window().ok_or(anyhow::anyhow!("Window does not exist"))
}

pub fn get_document() -> Result<Document> {
    get_window()?.document().ok_or(anyhow::anyhow!(
        "Document property does not exist on Window."
    ))
}

pub fn get_crypto() -> Result<Crypto> {
    wrap_js_err(get_window()?.crypto())
}

pub fn get_subtle_crypto() -> Result<SubtleCrypto> {
    Ok(get_crypto()?.subtle())
}

/// Redirects to a URL, if the URL is the same origin as the current one.
///
/// Supports replacing the URL and assiging the URL. Replacing a URL removes the current entry
/// from the browser history.
pub fn redirect(url: &str, replace: bool) -> Result<()> {
    let location = get_window()?.location();

    // The current origin (Protocol + Domain + Port)
    // Redirects are allowed, if they are redirecting to the same origin.
    let current_origin = location
        .origin()
        .map_err(|_| anyhow!("Cannot access current origin"))?;
    // Origin + Path
    let current_href = location
        .href()
        .map_err(|_| anyhow!("Cannot access current href"))?;

    let target_url =
        Url::new_with_base(url, &current_href).map_err(|_| anyhow!("Invalid URL format"))?;

    if target_url.origin() != current_origin {
        return Err(anyhow!("Cannot redirect outside origin"));
    }

    if replace {
        location
            .replace(url)
            .map_err(|_| anyhow!("Failed to redirect"))?;
    } else {
        location
            .assign(url)
            .map_err(|_| anyhow!("Failed to redirect"))?;
    }

    Ok(())
}
