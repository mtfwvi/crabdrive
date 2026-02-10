use crate::utils::browser::SessionStorage;
use anyhow::{Result, anyhow};
use argon2::{
    Algorithm, Argon2, Params, ParamsBuilder, PasswordHasher, Version, password_hash::Salt,
};
use base64::{Engine, prelude::BASE64_STANDARD};

/// Checks if a password has the minimum requirements.
/// Currently:
/// - Minimum 8 Characters
/// - At least one uppercase letter
/// - At least one number
/// - At least one special character
pub fn password_is_secure(password: &str) -> bool {
    if password.len() < 8 {
        return false;
    }

    let mut has_uppercase = false;
    let mut has_number = false;
    let mut has_special = false;

    for c in password.chars() {
        if c.is_uppercase() {
            has_uppercase = true;
        } else if c.is_numeric() {
            has_number = true;
        } else if !c.is_alphanumeric() {
            has_special = true;
        }

        if has_uppercase && has_number && has_special {
            return true;
        }
    }

    false
}

/// Creates a Base64 encoded
pub fn salt_from_username(username: &String) -> String {
    BASE64_STANDARD.encode(username)
}

pub fn get_argon2id_params() -> Params {
    let mut params_builder = ParamsBuilder::new();
    params_builder.output_len(32);
    params_builder.p_cost(1);
    params_builder.t_cost(3);
    params_builder.m_cost(1024 * 12);

    params_builder.build().unwrap()
}

/// Hash a password with a (Base64 encoded string) using Argon2Id. An (unsecure) salt can be generated using
/// [salt_from_username()].
///
/// Parameters are based on [OWASP Recommmendations](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#argon2id):
///  - Minimum Memory: 12 MiB
///  - Iterations: 3
///  - Parallelism Degree: 1
///
/// **This function is very computation-heavy, and may cause unresponsive UI!**
pub fn hash_password(password: &str, salt: &str) -> String {
    let salt = Salt::from_b64(salt).unwrap();

    let params = get_argon2id_params();
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let hash = argon2.hash_password(password.as_bytes(), salt).unwrap();
    hash.to_string()
}

/// Get the JWT Bearer token. Will return `Err` if no token is present.
pub fn get_token() -> Result<String> {
    SessionStorage::get("bearer")?.ok_or(anyhow!("Bearer token not found."))
}
