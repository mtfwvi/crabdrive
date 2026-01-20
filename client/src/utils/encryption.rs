use std::str::FromStr;
use crate::model::encryption::EncryptionKey;
use crate::model::node::DecryptedNode;
use crate::utils::file::{DecryptedChunk, EncryptedChunk};
use crabdrive_common::iv::IV;
use crabdrive_common::storage::{ChunkIndex, EncryptedNode};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::js_sys::ArrayBuffer;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{Array, JsString, Uint8Array};
use web_sys::{CryptoKey, SubtleCrypto};

fn get_subtle_crypto() -> SubtleCrypto {
    web_sys::window().unwrap().crypto().unwrap().subtle()
}

async fn get_key_from_bytes(key: EncryptionKey) -> CryptoKey {
    let format = "raw";
    let key_data = Uint8Array::new_from_slice(&key);
    let algorithm = "AES-GCM";
    let extractable = false;
    let key_usage = Array::new();
    key_usage.push(&JsString::from_str("encrypt").unwrap());
    key_usage.push(&JsString::from_str("decrypt").unwrap());

    JsFuture::from(get_subtle_crypto().import_key_with_str(
        format,
        &key_data,
        algorithm,
        extractable,
        &key_usage
    ).unwrap()).await.unwrap().dyn_into().unwrap()
}

pub async fn decrypt_node(
    node: EncryptedNode,
    key: EncryptionKey,
) -> Result<DecryptedNode, JsValue> {
    //TODO do actual decryption
    let decrypted_metadata = serde_json::from_slice(node.encrypted_metadata.as_slice()).unwrap();

    let decrypted_node = DecryptedNode {
        id: node.id,
        change_count: node.change_count,
        parent_id: node.parent_id,
        owner_id: node.owner_id,
        deleted_on: node.deleted_on,
        node_type: node.node_type,
        current_revision: node.current_revision,
        metadata: decrypted_metadata,
    };

    Ok(decrypted_node)
}

pub async fn encrypt_node(
    node: DecryptedNode,
    key: EncryptionKey,
) -> Result<EncryptedNode, JsValue> {
    //TODO do actual encryption

    let encrypted_metadata = serde_json::to_vec(&node.metadata).unwrap();

    let encrypted_node = EncryptedNode {
        id: node.id,
        change_count: node.change_count,
        parent_id: node.parent_id,
        owner_id: node.owner_id,
        deleted_on: node.deleted_on,
        node_type: node.node_type,
        current_revision: node.current_revision,
        encrypted_metadata,
        metadata_iv: IV::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    };

    Ok(encrypted_node)
}

pub async fn decrypt_chunk(
    chunk: &EncryptedChunk,
    key: EncryptionKey,
) -> Result<DecryptedChunk, JsValue> {
    let subtle_crypto  = get_subtle_crypto();


    //TODO actual encryption
    Ok(DecryptedChunk {
        chunk: chunk.chunk.clone(),
        index: chunk.index,
        first_block: chunk.first_block,
        last_block: chunk.last_block,
    })
}

pub async fn encrypt_chunk(
    chunk: &DecryptedChunk,
    key: EncryptionKey,
) -> Result<EncryptedChunk, JsValue> {
     //TODO actual encryption

    Ok(EncryptedChunk {
        chunk: chunk.chunk.clone(),
        index: chunk.index,
        first_block: chunk.first_block,
        last_block: chunk.last_block,

        iv_prefix: IV::new([0,0,0,0,0,0,0,0,0,0,0,0]),
    })
}

mod test {
    use wasm_bindgen_test::wasm_bindgen_test;
    use web_sys::js_sys::Uint8Array;
    use crate::constants::EMPTY_KEY;
    use crate::model::encryption::EncryptionKey;
    use crate::utils::encryption::{decrypt_chunk, encrypt_chunk, get_key_from_bytes};
    use crate::utils::file::DecryptedChunk;

    #[wasm_bindgen_test]
    async fn test_get_key_from_bytes() {
        get_key_from_bytes(EncryptionKey::default()).await;
    }
    #[wasm_bindgen_test]
    async fn test_encrypt_decrypt_chunk() {
        let example_buffer = vec![1,2,3,4,5,6,7,8,9];

        let chunk = DecryptedChunk {
            chunk: Uint8Array::new_from_slice(&example_buffer).buffer(),
            index: 0,
            first_block: false,
            last_block: false,
        };

        let encrypted_chunk = encrypt_chunk(&chunk, EMPTY_KEY).await.unwrap();

        let decrypted_chunk = decrypt_chunk(&encrypted_chunk, EMPTY_KEY).await.unwrap();

        let decrypted_chunk_array = Uint8Array::new(&decrypted_chunk.chunk).to_vec();

        assert_eq!(example_buffer, decrypted_chunk_array);
    }
}
