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
#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;
    use crate::utils::encryption::random::get_random_bytes;

    #[wasm_bindgen_test]
    fn test_get_random_bytes() {
        let random_bytes = get_random_bytes(32);
        assert_eq!(random_bytes.len(), 32);
        assert!(!random_bytes.eq(&[0; 32]));
    }
}
