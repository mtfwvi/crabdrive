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
    use crate::constants::EMPTY_KEY;
    use crate::model::encryption::EncryptionKey;
    use crate::model::node::{DecryptedNode, MetadataV1, NodeMetadata};
    use crate::utils::encryption::chunk::{decrypt_chunk, encrypt_chunk};
    use crate::utils::encryption::get_key_from_bytes;
    use crate::utils::encryption::node::{decrypt_metadata, decrypt_node, encrypt_metadata};
    use crate::utils::file::DecryptedChunk;
    use chrono::NaiveDateTime;
    use crabdrive_common::iv::IV;
    use crabdrive_common::storage::NodeId;
    use crabdrive_common::user::UserId;
    use wasm_bindgen_test::wasm_bindgen_test;
    use web_sys::js_sys::Uint8Array;

    #[wasm_bindgen_test]
    async fn test_get_key_from_bytes() {
        get_key_from_bytes(&EncryptionKey::default()).await;
    }

    #[wasm_bindgen_test]
    async fn test_encrypt_decrypt_chunk() {
        let example_buffer = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        let chunk = DecryptedChunk {
            chunk: Uint8Array::new_from_slice(&example_buffer).buffer(),
            index: 1,
            first_block: true,
            last_block: false,
        };

        let encrypted_chunk = encrypt_chunk(
            &chunk,
            &EMPTY_KEY,
            IV::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]),
        )
        .await
        .expect("encrypt chunk");

        let decrypted_chunk = decrypt_chunk(&encrypted_chunk, &EMPTY_KEY)
            .await
            .expect("decrypt chunk");

        let decrypted_chunk_array = Uint8Array::new(&decrypted_chunk.chunk).to_vec();

        assert_eq!(example_buffer, decrypted_chunk_array);
    }

    #[wasm_bindgen_test]
    async fn test_truncate_chunk() {
        let example_buffer = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        let chunk = DecryptedChunk {
            chunk: Uint8Array::new_from_slice(&example_buffer).buffer(),
            index: 2,
            first_block: false,
            last_block: false,
        };

        let mut encrypted_chunk = encrypt_chunk(
            &chunk,
            &EMPTY_KEY,
            IV::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]),
        )
        .await
        .expect("encrypt chunk");

        // server tries to truncate first block
        encrypted_chunk.index = 1;
        encrypted_chunk.first_block = true;

        let decrypted_chunk = decrypt_chunk(&encrypted_chunk, &EMPTY_KEY).await;

        assert!(decrypted_chunk.is_err())
    }

    #[wasm_bindgen_test]
    async fn test_encrypt_decrypt_metadata() {
        let example_metadata = NodeMetadata::V1(MetadataV1 {
                name: "hello.txt".to_string(),
                last_modified: NaiveDateTime::default(),
                created: NaiveDateTime::default(),
                size: None,
                mime_type: Some("txt".to_string()),
                file_key: None,
                children_key: vec![],
        });

        let encrypted_metadata = encrypt_metadata(&example_metadata, &EMPTY_KEY)
            .await
            .expect("could not encrypt node");


        let decrypted_metadata = decrypt_metadata(&encrypted_metadata, &EMPTY_KEY)
            .await
            .expect("could not decrypt node");

        assert_eq!(example_metadata, decrypted_metadata);
    }
}
