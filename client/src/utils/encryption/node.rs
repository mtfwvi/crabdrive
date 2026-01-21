use crate::constants::AES_GCM;
use crate::model::encryption::{DecryptedMetadata, EncryptedMetadata, EncryptionKey};
use crate::model::node::{DecryptedNode, NodeMetadata};
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
    let encrypted_metadata = EncryptedMetadata {
        data: node.encrypted_metadata,
        iv: node.metadata_iv
    };

    let decrypted_metadata = decrypt_metadata(&encrypted_metadata, &key).await?;

    let decrypted_node = DecryptedNode {
        id: node.id,
        change_count: node.change_count,
        parent_id: node.parent_id,
        owner_id: node.owner_id,
        deleted_on: node.deleted_on,
        node_type: node.node_type,
        current_revision: node.current_revision,
        metadata: decrypted_metadata,
        encryption_key: key
    };

    Ok(decrypted_node)
}

pub async fn decrypt_metadata(
    metadata: &EncryptedMetadata,
    key: &EncryptionKey,
) -> Result<NodeMetadata, JsValue> {
    let encrypted_metadata = Uint8Array::new_from_slice(&metadata.data);

    let subtle_crypto = encryption::get_subtle_crypto();
    let crypto_key = encryption::get_key_from_bytes(key).await;

    let iv_bytes = metadata.iv;
    let iv_bytes_array = Uint8Array::new_from_slice(&iv_bytes.get());
    let algorithm = AesGcmParams::new(AES_GCM, &iv_bytes_array);

    let decrypted_arraybuffer_promise = subtle_crypto.decrypt_with_object_and_buffer_source(
        &algorithm,
        &crypto_key,
        &encrypted_metadata,
    )?;
    let decrypted_arraybuffer: ArrayBuffer = JsFuture::from(decrypted_arraybuffer_promise)
        .await?
        .dyn_into()?;
    let decrypted_array = Uint8Array::new(&decrypted_arraybuffer);
    let decrypted_metadata = decrypted_array.to_vec();

    let metadata = serde_json::from_slice(&decrypted_metadata).unwrap();

    Ok(metadata)
}

pub async fn encrypt_metadata(
    metadata: &NodeMetadata,
    key: &EncryptionKey
) -> Result<EncryptedMetadata, JsValue> {
    let decrypted_metadata = serde_json::to_vec(metadata).unwrap();
    let decrypted_metadata_array = Uint8Array::new_from_slice(&decrypted_metadata);

    let iv: [u8; 12] = random::get_random_iv();

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

    Ok(EncryptedMetadata {
        data: encrypted_metadata,
        iv: IV::new(iv),
    })
}
