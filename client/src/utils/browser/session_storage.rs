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

    pub fn exists(key: &str) -> Result<bool> {
        let storage = Self::get_storage()?;

        Ok(storage
            .get_item(key)
            .map_err(|_| anyhow!("Cannot read from session storage"))?
            .is_some())
    }

    pub fn clear() -> Result<()> {
        let storage = Self::get_storage()?;
        storage
            .clear()
            .map_err(|_| anyhow!("Failed to clear session storage."))
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::browser::SessionStorage;
    use pretty_assertions::assert_eq;
    use test_case::test_case;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test_case("test", "test")]
    #[test_case("crabdrive", "#1")]
    #[test_case("horse", "cat")]
    #[test_case("ship", "car")]
    #[wasm_bindgen_test]
    async fn test_set_get(key: &str, value: &str) {
        SessionStorage::set(key, &value).expect("Expected to set value.");
        let got_value: Option<String> = SessionStorage::get(key).expect("Expected to get value.");
        assert!(got_value.is_some());
        assert_eq!(got_value.unwrap(), value);
    }

    #[wasm_bindgen_test]
    async fn test_clear() {
        SessionStorage::set("a", &"f").expect("Expected to set value.");
        SessionStorage::set("b", &"g").expect("Expected to set value.");
        SessionStorage::set("c", &"h").expect("Expected to set value.");
        SessionStorage::set("d", &"i").expect("Expected to set value.");
        SessionStorage::set("e", &"j").expect("Expected to set value.");
        SessionStorage::clear().expect("Expected to clear.");
        assert!(SessionStorage::get::<String>("a").unwrap().is_none());
        assert!(SessionStorage::get::<String>("b").unwrap().is_none());
        assert!(SessionStorage::get::<String>("c").unwrap().is_none());
        assert!(SessionStorage::get::<String>("d").unwrap().is_none());
        assert!(SessionStorage::get::<String>("e").unwrap().is_none());
    }
}
