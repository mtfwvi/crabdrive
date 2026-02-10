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

#[cfg(test)]
mod tests {
    use crate::utils::browser::LocalStorage;
    use pretty_assertions::assert_eq;
    use test_case::test_case;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test_case("test", "test")]
    #[test_case("crabdrive", "#1")]
    #[test_case("horse", "cat")]
    #[test_case("ship", "car")]
    #[wasm_bindgen_test]
    async fn test_set_get(key: &str, value: &str) {
        LocalStorage::set(key, &value).expect("Expected to set value.");
        let got_value: Option<String> = LocalStorage::get(key).expect("Expected to get value.");
        assert!(got_value.is_some());
        assert_eq!(got_value.unwrap(), value);
    }

    #[wasm_bindgen_test]
    async fn test_remove_item() {
        LocalStorage::set("a", &"f").expect("Expected to set value.");
        LocalStorage::set("b", &"g").expect("Expected to set value.");
        LocalStorage::set("c", &"h").expect("Expected to set value.");
        LocalStorage::set("d", &"i").expect("Expected to set value.");
        LocalStorage::set("e", &"j").expect("Expected to set value.");
        LocalStorage::remove("a").expect("Expected to clear.");
        LocalStorage::remove("b").expect("Expected to clear.");
        assert!(LocalStorage::get::<String>("a").unwrap().is_none());
        assert!(LocalStorage::get::<String>("b").unwrap().is_none());
        assert!(LocalStorage::get::<String>("c").unwrap().is_some());
        assert!(LocalStorage::get::<String>("d").unwrap().is_some());
        assert!(LocalStorage::get::<String>("e").unwrap().is_some());
    }

    #[wasm_bindgen_test]
    async fn test_clear() {
        LocalStorage::set("a", &"f").expect("Expected to set value.");
        LocalStorage::set("b", &"g").expect("Expected to set value.");
        LocalStorage::set("c", &"h").expect("Expected to set value.");
        LocalStorage::set("d", &"i").expect("Expected to set value.");
        LocalStorage::set("e", &"j").expect("Expected to set value.");
        LocalStorage::clear().expect("Expected to clear.");
        assert!(LocalStorage::get::<String>("a").unwrap().is_none());
        assert!(LocalStorage::get::<String>("b").unwrap().is_none());
        assert!(LocalStorage::get::<String>("c").unwrap().is_none());
        assert!(LocalStorage::get::<String>("d").unwrap().is_none());
        assert!(LocalStorage::get::<String>("e").unwrap().is_none());
    }
}
