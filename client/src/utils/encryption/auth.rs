use crate::model::encryption::DerivedKey;
use crate::model::encryption::{MasterKey, MetadataKey};
use crate::utils;
use crate::utils::browser::SessionStorage;

use anyhow::{Result, anyhow};
use argon2::{Algorithm, Argon2, ParamsBuilder, PasswordHasher, Version, password_hash::Salt};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;

/// Generates the server password, and the derived key from the password.
///
/// Returns `Result<(Password, DerivedKey)>`. The password is Base64-Encoded.
///
/// Parameters are based on [OWASP Recommmendations](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#argon2id):
///  - Minimum Memory: 12 MiB
///  - Iterations: 3
///  - Parallelism Degree: 1
///
/// **This function is very computation-heavy, and may cause unresponsive UI!**
pub fn derive_from_password(password_hash: &str, salt: &str) -> Result<(String, DerivedKey)> {
    tracing::debug_span!("encryption::utils::auth::deriveFromPassword");

    let salt = Salt::from_b64(salt)
        .map_err(|_| anyhow!("Invalid salt provided"))
        .inspect_err(|_| tracing::error!("Invalid salt! Is the username too short?"))?;

    let mut params_builder = ParamsBuilder::new();
    // First 32 bytes are Base-64 encoded into password
    // Last 32 bytes are derived key
    params_builder.output_len(64);
    params_builder.p_cost(1);
    params_builder.t_cost(3);
    params_builder.m_cost(1024 * 12);

    let params = params_builder.build().unwrap();
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let hash = argon2
        .hash_password(password_hash.as_bytes(), salt)
        .unwrap();

    let raw_bytes: [u8; 64] = hash.hash.unwrap().as_bytes().try_into()?;
    let split_bytes = raw_bytes.split_at(32);

    let password = BASE64_STANDARD.encode(split_bytes.0);
    let derived_key = split_bytes.1;

    Ok((password, derived_key.try_into()?))
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
