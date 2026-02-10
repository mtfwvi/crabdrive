use crate::utils::browser::SessionStorage;

use anyhow::{Result, anyhow};
use argon2::{
    Algorithm, Argon2, Params, ParamsBuilder, PasswordHasher, Version, password_hash::Salt,
};
use base64::Engine;
use base64::prelude::BASE64_STANDARD_NO_PAD;

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
pub fn salt_from_username(username: &str) -> String {
    // No padding, because argon2 returns `Err` if Base-64 encoded string contains `=`
    BASE64_STANDARD_NO_PAD.encode(username)
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
    // TODO: This seems to be incomaptible with Base64::encode() due to padding issues
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use test_case::test_case;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::utils;

    #[test_case("Crabdrive", false; "Only letters")]
    #[test_case("crabdrive123", false; "Only lowercase letters and numbers")]
    #[test_case("Crabdrive123", false; "Only letters and numbers")]
    #[test_case("CrAbDrIvE456!", true; "Letters, Numbers and Symbols")]
    fn check_password_is_secure(password: &str, expected: bool) {
        assert_eq!(utils::auth::password_is_secure(password), expected);
    }

    #[test_case("Crabdrive", "Q3JhYmRyaXZl")]
    #[test_case("evirdbraC", "ZXZpcmRicmFD")]
    #[test_case("CrabdriveIsBetterThanMega", "Q3JhYmRyaXZlSXNCZXR0ZXJUaGFuTWVnYQ")]
    #[wasm_bindgen_test]
    async fn test_username_to_salt(username: &str, expected: &str) {
        assert_eq!(utils::auth::salt_from_username(username), expected);
    }

    #[test_case(
        "crabdrive_is_best123",
        "crabdrive",
        "$argon2id$v=19$m=12288,t=3,p=1$Y3JhYmRyaXZlX2lzX2Jlc3QxMjM$UXqijhHYR91Hsl6OvUgLvLAN+EpJ7WpCKMqHlkJLyAU"
    )]
    #[test_case(
        "crabrive1!!1",
        "evirdbrac",
        "$argon2id$v=19$m=12288,t=3,p=1$Y3JhYnJpdmUxISEx$nfRWfSMytrLuj7Mm3IArmHTmBEr/IvepDhFBorIq4sU"
    )]
    #[test_case(
        "superuser",
        "argon2id",
        "$argon2id$v=19$m=12288,t=3,p=1$c3VwZXJ1c2Vy$MAzTKrUzxwJhykAtEBl6koU/uaSkkMSX2jQQh/S92wY"
    )]
    #[test_case(
        "haker123!",
        "superhacker",
        "$argon2id$v=19$m=12288,t=3,p=1$aGFrZXIxMjMh$X4UottU7VO91UV+wWCb4cAOlv5C1pD1hRsaGr3tBHL4"
    )]
    #[wasm_bindgen_test]
    async fn test_argon2id_vectors(username: &str, password: &str, expected: &str) {
        let salt = utils::auth::salt_from_username(username);
        assert_eq!(utils::auth::hash_password(password, &salt), expected);
    }
}
