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
use web_sys::{AesGcmParams, CryptoKey, SubtleCrypto};
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
    let additional_data = Uint8Array::new_from_slice(&[additional_byte]);

    let iv_bytes = chunk.iv_prefix.prefix_with_u32(chunk.index);
    let iv_bytes_array = Uint8Array::new_from_slice(&iv_bytes);
    let algorithm = AesGcmParams::new(AES_GCM, &iv_bytes_array);
    algorithm.set_additional_data(&additional_data);

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
    let additional_data = Uint8Array::new_from_slice(&[additional_byte]);
    let iv_bytes = iv_prefix.prefix_with_u32(chunk.index);
    let iv_bytes_array = Uint8Array::new_from_slice(&iv_bytes);
    let algorithm = AesGcmParams::new(AES_GCM, &iv_bytes_array);
    algorithm.set_additional_data(&additional_data);
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
    use crate::model::node::{DecryptedNode, MetadataV1, NodeMetadata};
    use crate::utils::encryption::{decrypt_chunk, encrypt_chunk, get_key_from_bytes};
    use crate::utils::file::DecryptedChunk;
    use chrono::{NaiveDate, NaiveDateTime};
    use crabdrive_common::storage::NodeId;
    use crabdrive_common::user::UserId;
    use wasm_bindgen_test::wasm_bindgen_test;
    use web_sys::js_sys::Uint8Array;
    use crabdrive_common::iv::IV;

    use super::{decrypt_node, encrypt_node};

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
    async fn test_encrypt_decrypt_node() {
        let example_node = DecryptedNode { 
            id: NodeId::random(), 
            change_count: 0, 
            parent_id: NodeId::random(), 
            owner_id: UserId::random(), 
            deleted_on: None, 
            node_type: crabdrive_common::storage::NodeType::Folder, 
            current_revision: None, 
            metadata: NodeMetadata::V1(MetadataV1 { 
                name: "hello.txt".to_string(), 
                last_modified: NaiveDateTime::default(), 
                created: NaiveDateTime::default(), 
                size: None, 
                mime_type: "txt".to_string(), 
                file_key: None, 
                children_key: vec![] })
        };

        let example_node_copy = example_node.clone();

        let encrypted_node = encrypt_node(example_node, EMPTY_KEY).await.expect("could not encrypt node");
        let decrypted_node = decrypt_node(encrypted_node, EMPTY_KEY).await.expect("could not decrypt node");

        assert_eq!(decrypted_node, example_node_copy);

    }


    // fn test_bytes_serialization() {
    //     let vec = vec![1,2,3,4,5,6,7,8,9,10,11,12];
    //     let vec_jsvalue = serde_wasm_bindgen::to_value(&vec).unwrap();
    //     panic!("{:?}" ,vec_jsvalue)
    // }
}
