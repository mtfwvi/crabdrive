use crate::utils::browser::get_crypto;
use crate::utils::error::{dyn_into, wrap_js_err};
use anyhow::Result;
use crabdrive_common::iv::IV;
use wasm_bindgen::JsValue;
use web_sys::js_sys::{Object, Uint8Array};

pub fn get_random_bytes(count: u32) -> Result<Vec<u8>> {
    let array = Uint8Array::new_with_length(count);

    // This should be a Uint8Array according to MDN
    let random_bytes_object: Object =
        wrap_js_err(get_crypto()?.get_random_values_with_js_u8_array(&array))?;

    let random_bytes_jsvalue: &JsValue = random_bytes_object.as_ref();

    let random_bytes: Uint8Array = dyn_into(random_bytes_jsvalue.clone())?;

    Ok(random_bytes.to_vec())
}

pub fn get_random_iv() -> Result<IV> {
    // unwrap seems safe
    let iv_bytes = get_random_bytes(12)?
        .try_into()
        .expect("vec has wrong size");
    Ok(IV::new(iv_bytes))
}

#[cfg(test)]
mod test {
    use crate::utils::encryption::random::{get_random_bytes, get_random_iv};
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_get_random_bytes() {
        let random_bytes = get_random_bytes(32).unwrap();
        assert_eq!(random_bytes.len(), 32);
        assert!(!random_bytes.eq(&[0; 32]));
    }

    #[wasm_bindgen_test]
    fn test_get_random_iv() {
        let iv = get_random_iv().unwrap();
        assert!(!iv.get().eq(&[0; 12]));
    }
}
