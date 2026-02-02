use crate::model::encryption::EncryptionKey;
use crate::utils::browser::get_subtle_crypto;
use crate::utils::error::{future_from_js_promise, wrap_js_err};
use anyhow::Result;
use wasm_bindgen::JsValue;
use web_sys::CryptoKey;
use web_sys::js_sys::{Array, Uint8Array};

pub mod chunk;
pub mod node;
pub mod random;

async fn get_key_from_bytes(key: &EncryptionKey) -> Result<CryptoKey> {
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

#[cfg(test)]
mod test {
    use crate::model::encryption::EncryptionKey;
    use crate::utils::encryption::get_key_from_bytes;

    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    async fn test_get_key_from_bytes() {
        get_key_from_bytes(&EncryptionKey::default()).await.unwrap();
    }
}
