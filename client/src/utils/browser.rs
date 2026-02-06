use crate::utils::error::wrap_js_err;
use anyhow::{Result, anyhow};
use web_sys::{Crypto, Document, SubtleCrypto, Window};

pub fn get_window() -> Result<Window> {
    if let Some(window) = web_sys::window() {
        Ok(window)
    } else {
        Err(anyhow!("window property does not exist"))
    }
}

pub fn get_document() -> Result<Document> {
    if let Some(document) = get_window()?.document() {
        Ok(document)
    } else {
        Err(anyhow!("document property does not exist"))
    }
}

pub fn get_crypto() -> Result<Crypto> {
    wrap_js_err(get_window()?.crypto())
}

pub fn get_subtle_crypto() -> Result<SubtleCrypto> {
    Ok(get_crypto()?.subtle())
}
