use crate::model::encryption::EncryptionKey;
use std::str::FromStr;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{Array, JsString, Uint8Array};
use web_sys::{CryptoKey, SubtleCrypto};

pub mod chunk;
pub mod node;
pub mod random;

fn get_subtle_crypto() -> SubtleCrypto {
    web_sys::window().unwrap().crypto().unwrap().subtle()
}

async fn get_key_from_bytes(key: &EncryptionKey) -> CryptoKey {
    let format = "raw";
    let key_data = Uint8Array::new_from_slice(key);
    let algorithm = "AES-GCM";
    let extractable = false;
    let key_usage = Array::new();
    key_usage.push(&JsString::from_str("encrypt").unwrap());
    key_usage.push(&JsString::from_str("decrypt").unwrap());

    JsFuture::from(
        get_subtle_crypto()
            .import_key_with_str(format, &key_data, algorithm, extractable, &key_usage)
            .unwrap(),
    )
    .await
    .unwrap()
    .dyn_into()
    .unwrap()
}

#[cfg(test)]
mod test {
    use crate::model::encryption::EncryptionKey;
    use crate::utils::encryption::get_key_from_bytes;

    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    async fn test_get_key_from_bytes() {
        get_key_from_bytes(&EncryptionKey::default()).await;
    }
}
