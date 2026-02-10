use anyhow::{Context, Result, anyhow};
use serde::{Serialize, de::DeserializeOwned};
use web_sys::Storage;

// Practically identical to session_storage.rs
/// Wrapper around local storage.
pub struct LocalStorage;

impl LocalStorage {
    fn get_storage() -> Result<Storage> {
        let window = super::get_window()?;

        window
            .local_storage()
            .map_err(|_| anyhow::anyhow!("Cannot access local storage"))?
            .ok_or(anyhow::anyhow!("Cannot access local storage"))
    }

    pub fn set<T: Serialize>(key: &str, value: &T) -> Result<()> {
        let storage = Self::get_storage()?;
        let json = serde_json::to_string(value).context("Cannot serialize")?;

        storage
            .set_item(key, &json)
            .map_err(|_| anyhow!("Cannot write to local storage"))?;

        Ok(())
    }

    pub fn get<T: DeserializeOwned>(key: &str) -> Result<Option<T>> {
        let storage = Self::get_storage()?;

        let json = storage
            .get_item(key)
            .map_err(|_| anyhow!("Cannot read from local storage"))?;

        match json {
            Some(data) => {
                let object = serde_json::from_str(&data).context("Cannot deserialize")?;
                Ok(Some(object))
            }
            None => Ok(None),
        }
    }

    pub fn exists(key: &str) -> Result<bool> {
        let storage = Self::get_storage()?;

        Ok(storage
            .get_item(key)
            .map_err(|_| anyhow!("Cannot read from local storage"))?
            .is_some())
    }

    pub fn remove(key: &str) -> Result<()> {
        let storage = Self::get_storage()?;

        storage
            .remove_item(key)
            .map_err(|_| anyhow!("Cannot remove from local storage"))
    }

    pub fn clear() -> Result<()> {
        let storage = Self::get_storage()?;
        storage
            .clear()
            .map_err(|_| anyhow!("Cannot clear local storage"))
    }
}
