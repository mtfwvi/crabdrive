use anyhow::{anyhow, Result};
use web_sys::Window;

pub fn get_window() -> Result<Window> {
    if let Some(window) = web_sys::window() {
        Ok(window)
    } else {
        Err(anyhow!("window property does not exist"))
    }
}