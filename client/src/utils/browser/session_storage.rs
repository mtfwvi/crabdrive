use anyhow::{Context, Result, anyhow};
use serde::{Serialize, de::DeserializeOwned};
use web_sys::Storage;

// The session storage is used to store Tokens and Encryption Keys.

/// Wrapper around session storage. Session storage is not-shared between threads,
pub struct SessionStorage;

impl SessionStorage {
    fn get_storage() -> Result<Storage> {
        let window = super::get_window()?;

        window
            .session_storage()
            .map_err(|_| anyhow!("Cannot access session storage."))?
            .ok_or(anyhow!("Cannot access session storage."))
    }

    pub fn set<T: Serialize>(key: &str, value: &T) -> Result<()> {
        let storage = Self::get_storage()?;
        let json = serde_json::to_string(value).context("Cannot serialize.")?;

        storage
            .set_item(key, &json)
            .map_err(|_| anyhow!("Cannot write to session storage."))?;

        Ok(())
    }

    pub fn get<T: DeserializeOwned>(key: &str) -> Result<Option<T>> {
        let storage = Self::get_storage()?;

        let json = storage
            .get_item(key)
            .map_err(|_| anyhow!("Cannot read from session storage."))?;

        match json {
            Some(data) => {
                let object = serde_json::from_str(&data).context("Cannot deserialize.")?;
                Ok(Some(object))
            }
            None => Ok(None),
        }
    }

    /// Clears all data inside the session storage.
    pub fn clear() -> Result<()> {
        let storage = Self::get_storage()?;
        storage
            .clear()
            .map_err(|_| anyhow!("Failed to clear session storage."))
    }
}
