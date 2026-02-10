use crate::model::encryption::DerivedKey;
use crate::model::encryption::{MasterKey, MetadataKey};
use crate::utils;
use crate::utils::browser::SessionStorage;

use anyhow::{Result, anyhow};
use argon2::{Algorithm, Argon2, PasswordHasher, Version, password_hash::Salt};

/// Derives the key (used for wrapping / unwrapping the master key) from a password
pub async fn derive_from_password(password_hash: &str, salt: &str) -> Result<DerivedKey> {
    let salt = Salt::from_b64(salt).unwrap();

    let params = utils::auth::get_argon2id_params();
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let hash = argon2
        .hash_password(password_hash.as_bytes(), salt)
        .unwrap();
    let raw_key: DerivedKey = hash.hash.unwrap().as_bytes().try_into()?;

    Ok(raw_key)
}

/// Get the root token. Will return `Err` if no token is present.
pub fn get_root_key() -> Result<MetadataKey> {
    let root_key: String = SessionStorage::get("root_key")?
        .ok_or(anyhow!("Invalid encryption key. Please re-authenticate"))?;
    utils::encryption::decode_key(&root_key)
}

/// Get the trash token. Will return `Err` if no token is present.
pub fn get_trash_key() -> Result<MetadataKey> {
    let trash_key: String = SessionStorage::get("trash_key")?
        .ok_or(anyhow!("Invalid encryption key. Please re-authenticate"))?;
    utils::encryption::decode_key(&trash_key)
}

/// Get the master token. Will return `Err` if no token is present.
pub fn get_master_key() -> Result<MasterKey> {
    let master_key: String = SessionStorage::get("master_key")?
        .ok_or(anyhow!("Invalid encryption key. Please re-authenticate"))?;
    utils::encryption::decode_key(&master_key)
}
