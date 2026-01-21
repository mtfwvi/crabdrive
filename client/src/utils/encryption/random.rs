use crate::model::encryption::EncryptionKey;
use wasm_bindgen::JsCast;
use web_sys::js_sys::Uint8Array;

pub fn get_random_bytes(count: u32) -> Vec<u8> {
    let window = web_sys::window().expect("window does not exist");
    let array = Uint8Array::new_with_length(count);
    let random_bytes: Uint8Array = window
        .crypto()
        .unwrap()
        .get_random_values_with_js_u8_array(&array)
        .expect("Failed to get random bytes")
        .dyn_into()
        .unwrap();
    random_bytes.to_vec()
}

pub fn get_random_iv() -> [u8; 12] {
    get_random_bytes(12).try_into().unwrap()
}

pub fn get_random_encryption_key() -> EncryptionKey {
    get_random_bytes(32).try_into().unwrap()
}

#[cfg(test)]
mod test {
    use crate::utils::encryption::random::{
        get_random_bytes, get_random_encryption_key, get_random_iv,
    };
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_get_random_bytes() {
        let random_bytes = get_random_bytes(32);
        assert_eq!(random_bytes.len(), 32);
        assert!(!random_bytes.eq(&[0; 32]));
    }

    #[wasm_bindgen_test]
    fn test_get_random_iv() {
        let iv = get_random_iv();
        assert!(!iv.eq(&[0; 12]));
    }

    #[wasm_bindgen_test]
    fn test_get_random_encryption_key() {
        let iv = get_random_encryption_key();
        assert!(!iv.eq(&[0; 32]));
    }
}
