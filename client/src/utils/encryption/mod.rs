use crate::model::encryption::EncryptionKey;
use crate::model::node::DecryptedNode;
use crate::utils::file::{DecryptedChunk, EncryptedChunk};
use crabdrive_common::iv::{IvWithPrefix, IV};
use crabdrive_common::storage::EncryptedNode;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::js_sys::ArrayBuffer;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{Array, JsString, Object, Uint8Array};
use web_sys::{CryptoKey, SubtleCrypto};
use crate::constants::AES_GCM;

mod random;

fn get_subtle_crypto() -> SubtleCrypto {
    web_sys::window().unwrap().crypto().unwrap().subtle()
}

fn get_additional_byte(first_chunk: bool, last_chunk: bool) -> u8 {
    match (first_chunk, last_chunk) {
        (true, false) => 1, // start chunk
        (false, true) => 2, // end chunk
        (true, true) => 3, // only chunk
        (false, false) => 4, // in between chunk

    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AesGcmParamsChunk {
    iv: IvWithPrefix,
    name: &'static str,
    //additionalData: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AesGcmParamsMetadata {
    iv: IV,
    name: &'static str,
}

impl AesGcmParamsChunk {
    pub fn new(iv: IvWithPrefix, additional_byte: u8) -> Self {
        Self {
            iv,
            name: AES_GCM,
            //additionalData: vec![additional_byte],
        }
    }

    pub fn to_object(&self) -> Result<Object, JsValue> {
        serde_wasm_bindgen::to_value(self)?.dyn_into::<Object>()
    }
}

impl AesGcmParamsMetadata {
    pub fn new(iv: IV) -> Self {
        Self {
            iv,
            name: AES_GCM,
        }
    }

    pub fn to_object(&self) -> Result<Object, JsValue> {
        serde_wasm_bindgen::to_value(self)?.dyn_into::<Object>()
    }
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
    let key = get_key_from_bytes(key).await;

    let additional_byte = get_additional_byte(chunk.first_block, chunk.last_block);
    let iv_bytes = chunk.iv_prefix.prefix_with_u32(chunk.index);
    let algorithm = AesGcmParamsChunk::new(iv_bytes, additional_byte).to_object()?;

    let decrypted_arraybuffer_promise = subtle_crypto.decrypt_with_object_and_buffer_source(&algorithm, &key, &chunk.chunk)?;
    let decrypted_arraybuffer: ArrayBuffer = JsFuture::from(decrypted_arraybuffer_promise).await?.dyn_into()?;

    Ok(DecryptedChunk {
        chunk: decrypted_arraybuffer,
        index: chunk.index,
        first_block: chunk.first_block,
        last_block: chunk.last_block,
    })
}

pub async fn encrypt_chunk(
    chunk: &DecryptedChunk,
    key: EncryptionKey,
    iv_prefix: IV
) -> Result<EncryptedChunk, JsValue> {
    let subtle_crypto  = get_subtle_crypto();
    let key = get_key_from_bytes(key).await;

    let additional_byte = get_additional_byte(chunk.first_block, chunk.last_block);
    let iv_bytes = iv_prefix.prefix_with_u32(chunk.index);
    let algorithm = AesGcmParamsChunk::new(iv_bytes, additional_byte).to_object()?;
    let encrypted_arraybuffer_promise = subtle_crypto.encrypt_with_object_and_buffer_source(&algorithm, &key, &chunk.chunk)?;
    let encrypted_arraybuffer: ArrayBuffer = JsFuture::from(encrypted_arraybuffer_promise).await?.dyn_into()?;

    Ok(EncryptedChunk {
        chunk: encrypted_arraybuffer,
        index: chunk.index,
        first_block: chunk.first_block,
        last_block: chunk.last_block,
        iv_prefix,
    })
}

#[cfg(test)]
mod test {
    use crate::constants::EMPTY_KEY;
    use crate::model::encryption::EncryptionKey;
    use crate::utils::encryption::{decrypt_chunk, encrypt_chunk, get_key_from_bytes};
    use crate::utils::file::DecryptedChunk;
    use wasm_bindgen_test::wasm_bindgen_test;
    use web_sys::js_sys::Uint8Array;
    use crabdrive_common::iv::IV;

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
            last_block: false
        };

        let encrypted_chunk = encrypt_chunk(&chunk, EMPTY_KEY, IV::new([1,2,3,4,5,6,7,8,9,10,11,12])).await.expect("encrypt chunk");

        let decrypted_chunk = decrypt_chunk(&encrypted_chunk, EMPTY_KEY).await.expect("decrypt chunk");

        let decrypted_chunk_array = Uint8Array::new(&decrypted_chunk.chunk).to_vec();

        assert_eq!(example_buffer, decrypted_chunk_array);
    }

    #[wasm_bindgen_test]
    fn test_bytes_serialization() {
        let vec = vec![1,2,3,4,5,6,7,8,9,10,11,12];
        let vec_jsvalue = serde_wasm_bindgen::to_value(&vec).unwrap();
        panic!("{:?}" ,vec_jsvalue)
    }
}
