pub mod auth;
pub mod chunk;
pub mod node;
pub mod random;

use crate::hex_fmt;
use crate::model::encryption::{DerivedKey, RawEncryptionKey, WrappedKey};
use crate::utils::browser::get_subtle_crypto;
use crate::utils::error::{future_from_js_promise, wrap_js_err};

use anyhow::{Result, anyhow};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use crabdrive_common::encryption_key::EncryptionKey;
use crabdrive_common::iv::IV;
use wasm_bindgen::JsValue;
use web_sys::js_sys::{Array, Uint8Array};
use web_sys::{AesGcmParams, AesKeyGenParams, CryptoKey};

/// Create a SHA256 digest from input
pub async fn sha256_digest(data: &str) -> Result<[u8; 32]> {
    let _guard = tracing::trace_span!("utils::encryption::sha256Digest").entered();

    let crypto = get_subtle_crypto()?;
    let data = data.as_bytes();

    let digest_hash = future_from_js_promise(wrap_js_err(
        crypto.digest_with_str_and_u8_array("SHA-256", data),
    )?)
    .await?;

    let mut hash = [0u8; 32];
    let array = Uint8Array::new(&digest_hash);
    array.copy_to(&mut hash);

    Ok(hash)
}

pub async fn import_key(key: &RawEncryptionKey) -> Result<CryptoKey> {
    let _guard = tracing::trace_span!("utils::encryption::importKey").entered();

    let format = "raw";
    let key_data = Uint8Array::new_from_slice(key);
    let algorithm = "AES-GCM";
    let extractable = true;
    let key_usage = Array::new();
    key_usage.push(&JsValue::from("encrypt"));
    key_usage.push(&JsValue::from("decrypt"));
    key_usage.push(&JsValue::from("wrapKey"));
    key_usage.push(&JsValue::from("unwrapKey"));

    let key_promise = wrap_js_err(get_subtle_crypto()?.import_key_with_str(
        format,
        &key_data,
        algorithm,
        extractable,
        &key_usage,
    ))?;

    let crypto_key = future_from_js_promise(key_promise).await?;
    tracing::trace!("Imported key: {}", hex_fmt!(key));

    Ok(crypto_key)
}
pub async fn export_key(key: &CryptoKey) -> Result<RawEncryptionKey> {
    let _guard = tracing::trace_span!("utils::encryption::exportKey").entered();

    let format = "raw";

    let key_promise = get_subtle_crypto()?
        .export_key(format, key)
        .map_err(|_| anyhow!("Failed to export key!"))?;
    let key_buffer = future_from_js_promise(key_promise).await?;
    let key_array = Uint8Array::new(&key_buffer);
    let key_vec = key_array.to_vec();
    let raw_key: [u8; 32] = key_vec
        .try_into()
        .map_err(|_| anyhow!("Failed to export key!"))?;

    tracing::trace!("Exported Key: {}", hex_fmt!(raw_key));

    Ok(raw_key)
}

pub fn decode_key(key: &str) -> Result<RawEncryptionKey> {
    let key_bytes = BASE64_STANDARD
        .decode(key)
        .map_err(|_| anyhow!("Failed to decode key!"))?;
    let raw_key: RawEncryptionKey = key_bytes
        .try_into()
        .map_err(|_| anyhow!("Failed to import key!"))?;
    Ok(raw_key)
}

pub fn encode_key(key: &RawEncryptionKey) -> String {
    BASE64_STANDARD.encode(key)
}

/// Generates a random AES-GCM 256 key (used for master, root & trash keys)
pub async fn generate_aes256_key() -> Result<RawEncryptionKey> {
    let _guard = tracing::trace_span!("utils::encryption::generateAes256Key").entered();

    let crypto = get_subtle_crypto()?;

    let usages = Array::new();
    usages.push(&JsValue::from("encrypt"));
    usages.push(&JsValue::from("decrypt"));

    let params = AesKeyGenParams::new("AES-GCM", 256);

    let key: CryptoKey = future_from_js_promise(
        crypto
            .generate_key_with_object(&params, true, &usages)
            .map_err(|_| anyhow!("Failed to generate master key"))?,
    )
    .await?;

    let raw_key = export_key(&key).await?;
    tracing::trace!("Generated key: {}", hex_fmt!(raw_key));

    Ok(raw_key)
}

/// Unwraps a key from the wrapped key
pub async fn unwrap_key(
    wrapped_key: &WrappedKey,
    derived_key: &DerivedKey,
) -> Result<RawEncryptionKey> {
    let crypto = get_subtle_crypto()?;

    let iv_bytes = Uint8Array::new_from_slice(&wrapped_key.iv().get());
    // Import both keys
    let wrapped_key = Uint8Array::from(wrapped_key.key_slice()).buffer();
    let derived_key = import_key(derived_key).await?;

    let params = AesGcmParams::new("AES-GCM", &iv_bytes);
    let usages = Array::new();
    usages.push(&JsValue::from("encrypt"));
    usages.push(&JsValue::from("decrypt"));

    let _guard = tracing::trace_span!("utils::encryption::unwrapKey").entered();

    let key: CryptoKey = future_from_js_promise(
        crypto
            .unwrap_key_with_buffer_source_and_object_and_str(
                "raw",
                &wrapped_key,
                &derived_key,
                &params,
                "AES-GCM",
                true,
                &usages,
            )
            .map_err(|_| anyhow!("Cannot unwrap key!"))?,
    )
    .await?;

    let key = export_key(&key).await?;
    tracing::trace!("Successfully unwrapped key: {}", hex_fmt!(key));

    Ok(key)
}

/// Wraps the master / root / trash / private key into a key.
pub async fn wrap_key(
    master_key: &RawEncryptionKey,
    derived_key: &DerivedKey,
) -> Result<WrappedKey> {
    let _guard = tracing::trace_span!("utils::encryption::wrapKey").entered();

    let crypto = get_subtle_crypto()?;
    let iv: IV = random::get_random_iv()?;
    let iv_bytes = Uint8Array::new_from_slice(&iv.get());

    let master_key = import_key(master_key).await?;
    let derived_key = import_key(derived_key).await?;

    let params = AesGcmParams::new("AES-GCM", &iv_bytes);

    let wrapped_key: JsValue = future_from_js_promise(
        crypto
            .wrap_key_with_object("raw", &master_key, &derived_key, &params)
            .map_err(|_| anyhow!("Cannot wrap key!"))?,
    )
    .await?;

    let key = Uint8Array::new(&wrapped_key).to_vec();
    tracing::trace!("Successfully wrapped key: {}", hex_fmt!(key));

    Ok(EncryptionKey::new(key, iv))
}

#[macro_export]
/// Formats a `[u8]` as hex
macro_rules! hex_fmt {
    ($bytes:expr) => {
        $bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    };
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use test_case::test_case;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::model::encryption::RawEncryptionKey;
    use crate::utils;

    async fn get_random_test_key() -> RawEncryptionKey {
        utils::encryption::generate_aes256_key()
            .await
            .expect("Failed to generate key")
    }

    #[wasm_bindgen_test]
    async fn test_get_key_from_bytes() {
        utils::encryption::import_key(&RawEncryptionKey::default())
            .await
            .unwrap();
    }

    #[test_case("", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")]
    #[test_case(
        "abc",
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    )]
    #[test_case(
        "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
        "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
    )]
    #[wasm_bindgen_test]
    async fn test_sha256_digest(value: &str, expected: &str) {
        assert_eq!(
            hex_fmt!(utils::encryption::sha256_digest(value).await.unwrap()),
            expected
        );
    }

    #[wasm_bindgen_test]
    async fn test_generate_aes256_key() {
        let result = utils::encryption::generate_aes256_key().await;
        assert!(result.is_ok(), "Should generate a key successfully");

        let key = result.unwrap();
        assert_eq!(key.len(), 32, "AES-256 key should be 32 bytes");
        assert_ne!(key, [0u8; 32], "Key should not be zeroed out");
    }

    #[wasm_bindgen_test]
    async fn test_encode_decode_key() {
        let original_key = get_random_test_key().await;
        let encoded = utils::encryption::encode_key(&original_key);
        assert!(!encoded.is_empty(), "Encoded string should not be empty");
        let decoded = utils::encryption::decode_key(&encoded).expect("Failed to decode key");
        assert_eq!(original_key, decoded, "Decoded key should match original");
    }

    #[wasm_bindgen_test]
    async fn test_import_export() {
        let original_raw = get_random_test_key().await;

        let crypto_key = utils::encryption::import_key(&original_raw)
            .await
            .expect("Should import raw key");

        let exported_raw = utils::encryption::export_key(&crypto_key)
            .await
            .expect("Should export crypto key");

        assert_eq!(
            original_raw, exported_raw,
            "Exported key should match imported key"
        );
    }

    #[wasm_bindgen_test]
    async fn test_wrap_unwrap_flow() {
        let master_key_raw = get_random_test_key().await;
        let derived_key_raw = get_random_test_key().await;

        let wrapped_struct = utils::encryption::wrap_key(&master_key_raw, &derived_key_raw)
            .await
            .expect("Should wrap key successfully");
        let unwrapped_key_raw = utils::encryption::unwrap_key(&wrapped_struct, &derived_key_raw)
            .await
            .expect("Should unwrap key successfully");

        assert_eq!(
            master_key_raw, unwrapped_key_raw,
            "The unwrapped key must match the orignal key"
        );
    }
}
