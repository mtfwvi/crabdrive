use crate::utils::browser::{LocalStorage, SessionStorage, get_current_url, get_origin, redirect};

use crate::utils;
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
/// - Minimum 12 Characters
pub fn is_valid_password(password: &str) -> bool {
    password.len() >= 12
}

/// Checks if a username is valid
/// Currently:
/// - Minimum 3 Characters
/// - Maximum 32 Characters
pub fn is_valid_username(username: &str) -> bool {
    let len = username.trim().len();
    len > 2 && len <= 32
}

/// Creates a Base64 encoded
pub async fn salt_from_username(username: &str) -> String {
    // Hash the username to prevent issues with salts, that are too short
    let username = utils::encryption::sha256_digest(username).await.unwrap();
    // No padding, because argon2 returns `Err` if Base-64 encoded string contains `=`
    BASE64_STANDARD_NO_PAD.encode(username)
}

/// Get the JWT Bearer token. Will return `Err` if no token is present but should redirect to the login page.
pub fn get_token() -> Result<String> {
    if let Some(token) = SessionStorage::get("bearer")? {
        Ok(token)
    } else {
        go_to_login()?;
        Err(anyhow!("Bearer token not found."))
    }
}

pub fn go_to_login() -> Result<()> {
    SessionStorage::clear()?;
    let current_url = get_current_url()?;

    // without this, the login page redirects to the login page for some reason (/ -> /login -> /login)
    // if we are on the login page, it is not necessary to store the url as it would overwrite the old one
    if current_url.contains("login") {
        return Ok(());
    }

    LocalStorage::set("redirect_url", &current_url)?;

    let mut login_url = get_origin()?;
    login_url.push_str("/login");

    redirect(&login_url, false)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use test_case::test_case;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::utils;

    #[test_case("", false; "0 Characters")]
    #[test_case("Crabdrive", false; "9 Characters")]
    #[test_case("Crabdrive123", true; "12 Characters")]
    #[test_case("CrAbDrIvE456!", true; "13 Characters")]
    fn check_password_is_secure(password: &str, expected: bool) {
        assert_eq!(utils::auth::is_valid_password(password), expected);
    }

    #[test_case("crab", "M0mpFffNDC/yXkuO5Or5Zbod6cJQYuZJ5NlPgTfOk0I")]
    #[test_case("Crabdrive", "xp3+RiYezQ7EyYuEdJuvyVqa8mB2KOnC5gSLisivBMw")]
    #[test_case("evirdbraC", "vmyMloVQnrf2q6tXLe+/liXwj2Gyi3OC2HPj6pqJzxM")]
    #[test_case(
        "CrabdriveIsBetterThanMega",
        "C2NciVB5nXQCCcxR+riz8iJc39GysynTxyMRNPxUnVk"
    )]
    #[wasm_bindgen_test]
    async fn test_username_to_salt(username: &str, expected: &str) {
        assert_eq!(utils::auth::salt_from_username(username).await, expected);
    }
}
