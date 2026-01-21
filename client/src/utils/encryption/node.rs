use crate::constants::AES_GCM;
use crate::model::encryption::EncryptionKey;
use crate::model::node::DecryptedNode;
use crate::utils::encryption;
use crate::utils::encryption::random;
use crabdrive_common::iv::IV;
use crabdrive_common::storage::EncryptedNode;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_futures::js_sys::{ArrayBuffer, Uint8Array};
use web_sys::AesGcmParams;

pub async fn decrypt_node(
    node: EncryptedNode,
    key: EncryptionKey,
) -> Result<DecryptedNode, JsValue> {
    let encrypted_metadata = Uint8Array::new_from_slice(&node.encrypted_metadata);

    let subtle_crypto = encryption::get_subtle_crypto();
    let key = encryption::get_key_from_bytes(key).await;

    let iv_bytes = node.metadata_iv;
    let iv_bytes_array = Uint8Array::new_from_slice(&iv_bytes.get());
    let algorithm = AesGcmParams::new(AES_GCM, &iv_bytes_array);

    let decrypted_arraybuffer_promise = subtle_crypto.decrypt_with_object_and_buffer_source(
        &algorithm,
        &key,
        &encrypted_metadata,
    )?;
    let decrypted_arraybuffer: ArrayBuffer = JsFuture::from(decrypted_arraybuffer_promise)
        .await?
        .dyn_into()?;
    let decrypted_array = Uint8Array::new(&decrypted_arraybuffer);
    let decrypted_metadata = decrypted_array.to_vec();

    let metadata = serde_json::from_slice(&decrypted_metadata).unwrap();

    let decrypted_node = DecryptedNode {
        id: node.id,
        change_count: node.change_count,
        parent_id: node.parent_id,
        owner_id: node.owner_id,
        deleted_on: node.deleted_on,
        node_type: node.node_type,
        current_revision: node.current_revision,
        metadata,
    };

    Ok(decrypted_node)
}

pub async fn encrypt_node(
    node: DecryptedNode,
    key: EncryptionKey,
) -> Result<EncryptedNode, JsValue> {
    let decrypted_metadata = serde_json::to_vec(&node.metadata).unwrap();
    let decrypted_metadata_array = Uint8Array::new_from_slice(&decrypted_metadata);

    let iv: [u8; 12] = random::get_random_bytes(12).try_into().unwrap();

    let subtle_crypto = encryption::get_subtle_crypto();
    let key = encryption::get_key_from_bytes(key).await;

    let iv_bytes_array = Uint8Array::new_from_slice(&iv);
    let algorithm = AesGcmParams::new(AES_GCM, &iv_bytes_array);

    let encrypted_arraybuffer_promise = subtle_crypto.encrypt_with_object_and_buffer_source(
        &algorithm,
        &key,
        &decrypted_metadata_array,
    )?;
    let encrypted_arraybuffer: ArrayBuffer = JsFuture::from(encrypted_arraybuffer_promise)
        .await?
        .dyn_into()?;
    let encrypted_array = Uint8Array::new(&encrypted_arraybuffer);
    let encrypted_metadata = encrypted_array.to_vec();

    let encrypted_node = EncryptedNode {
        id: node.id,
        change_count: node.change_count,
        parent_id: node.parent_id,
        owner_id: node.owner_id,
        deleted_on: node.deleted_on,
        node_type: node.node_type,
        current_revision: node.current_revision,
        encrypted_metadata,
        metadata_iv: IV::new(iv),
    };

    Ok(encrypted_node)
}
