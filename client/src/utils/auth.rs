use crate::utils::browser::{LocalStorage, SessionStorage};

use anyhow::{Result, anyhow};
use base64::Engine;
use base64::prelude::BASE64_STANDARD_NO_PAD;

/// Checks if a username is remembered
pub fn is_authenticated() -> Result<bool> {
    Ok(SessionStorage::exists("username")? && SessionStorage::exists("bearer")?)
}

/// Checks if a username is remembered
pub fn get_username() -> Result<String> {
    if !is_authenticated()? {
        SessionStorage::clear()?;
        return Err(anyhow!("Unauthenticated"));
    }

    LocalStorage::get::<String>("username")?.ok_or(anyhow!("Unauthenticated"))
}

/// Get the last used username for signing in
pub fn get_last_used_username() -> Result<Option<String>> {
    LocalStorage::get::<String>("last_user")
}

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
}
