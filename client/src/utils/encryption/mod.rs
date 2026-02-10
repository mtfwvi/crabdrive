pub mod auth;
pub mod chunk;
pub mod node;
pub mod random;

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

async fn import_key(key: &RawEncryptionKey) -> Result<CryptoKey> {
    let format = "raw";
    let key_data = Uint8Array::new_from_slice(key);
    let algorithm = "AES-GCM";
    let extractable = false;
    let key_usage = Array::new();
    key_usage.push(&JsValue::from("encrypt"));
    key_usage.push(&JsValue::from("decrypt"));

    let key_promise = wrap_js_err(get_subtle_crypto()?.import_key_with_str(
        format,
        &key_data,
        algorithm,
        extractable,
        &key_usage,
    ))?;

    future_from_js_promise(key_promise).await
}

pub async fn export_key(key: &CryptoKey) -> Result<RawEncryptionKey> {
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

    Ok(raw_key)
}

pub fn decode_key(key: &String) -> Result<RawEncryptionKey> {
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
    let crypto = get_subtle_crypto()?;

    let params = AesKeyGenParams::new("AES-GCM", 256);
    let key: CryptoKey = future_from_js_promise(
        crypto
            .generate_key_with_object(
                &params,
                true,
                &Array::new(), // The key is extracted directly, so no key usages are needed.
            )
            .map_err(|_| anyhow!("Failed to generate master key"))?,
    )
    .await?;

    let raw_key = export_key(&key).await?;
    Ok(raw_key)
}

/// Unwraps a key from the wrapped key
pub async fn unwrap_key(
    wrapped_key: WrappedKey,
    derived_key: DerivedKey,
) -> Result<RawEncryptionKey> {
    let crypto = get_subtle_crypto()?;

    let iv_bytes = Uint8Array::new_from_slice(&wrapped_key.iv().get());
    // Import both keys
    let wrapped_key = Uint8Array::from(wrapped_key.key_slice()).buffer();
    let derived_key = import_key(&derived_key).await?;

    let params = AesGcmParams::new("AES-GCM", &iv_bytes);

    let key: CryptoKey = future_from_js_promise(
        crypto
            .unwrap_key_with_buffer_source_and_object_and_str(
                "raw",
                &wrapped_key,
                &derived_key,
                &params,
                "AES-GCM",
                true,
                &Array::new(), // The key is extracted directly, so no key usages are needed.
            )
            .map_err(|_| anyhow!("Cannot unwrap key!"))?,
    )
    .await?;
    let key = export_key(&key).await?;

    Ok(key)
}

/// Wraps the master / root / trash / private key into a key.
pub async fn wrap_key(
    master_key: RawEncryptionKey,
    derived_key: &DerivedKey,
) -> Result<WrappedKey> {
    let crypto = get_subtle_crypto()?;

    let iv: IV = random::get_random_iv()?;
    let iv_bytes = Uint8Array::new_from_slice(&iv.get());

    let master_key = import_key(&master_key).await?;
    let derived_key = import_key(derived_key).await?;

    let params = AesGcmParams::new("AES-GCM", &iv_bytes);

    let key: CryptoKey = future_from_js_promise(
        crypto
            .wrap_key_with_object("raw", &master_key, &derived_key, &params)
            .map_err(|_| anyhow!("Cannot wrap key!"))?,
    )
    .await?;
    let key = export_key(&key).await?;

    Ok(EncryptionKey::new(key.into(), iv))
}

#[cfg(test)]
mod test {
    use crate::model::encryption::RawEncryptionKey;
    use crate::utils::encryption::import_key;

    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    async fn test_get_key_from_bytes() {
        import_key(&RawEncryptionKey::default()).await.unwrap();
    }
}
