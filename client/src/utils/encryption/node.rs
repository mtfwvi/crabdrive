use crate::constants::AES_GCM;
use crate::model::encryption::MetadataKey;
use crate::model::node::{DecryptedNode, NodeMetadata};
use crate::utils::browser::get_subtle_crypto;
use crate::utils::encryption::{import_key, random};
use crate::utils::error::wrap_js_err;

use crabdrive_common::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::iv::IV;
use crabdrive_common::storage::EncryptedNode;

use anyhow::{Error, anyhow, Result};
use tracing::debug_span;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_futures::js_sys::{ArrayBuffer, Uint8Array};
use web_sys::AesGcmParams;

pub async fn decrypt_node(
    node: EncryptedNode,
    metadata_key: MetadataKey,
) -> Result<DecryptedNode, Error> {
    let _guard = debug_span!("utils::encryption::decryptNode").entered();

    let decrypted_metadata = decrypt_metadata(&node.encrypted_metadata, &metadata_key)
        .await
        .inspect_err(|_| tracing::error!("Failed to decrypt metadata ({})!", node.id))?;

    let decrypted_node = DecryptedNode {
        id: node.id,
        change_count: node.change_count,
        parent_id: node.parent_id,
        owner_id: node.owner_id,
        deleted_on: node.deleted_on,
        node_type: node.node_type,
        current_revision: node.current_revision,
        metadata: decrypted_metadata,
        encryption_key: metadata_key,
    };

    Ok(decrypted_node)
}

pub async fn decrypt_metadata(
    metadata: &EncryptedMetadata,
    metadata_key: &MetadataKey,
) -> Result<NodeMetadata, Error> {
    let _guard = debug_span!("utils::encryption::decryptNodeMetadata").entered();

    let encrypted_metadata = Uint8Array::new_from_slice(metadata.metadata());

    let subtle_crypto = get_subtle_crypto()?;
    let crypto_key = import_key(metadata_key)
        .await
        .inspect_err(|e| tracing::error!("Failed to import key: {}", e))?;

    let iv_bytes = metadata.iv();
    let iv_bytes_array = Uint8Array::new_from_slice(&iv_bytes.get());
    let algorithm = AesGcmParams::new(AES_GCM, &iv_bytes_array);

    let decrypted_arraybuffer_promise =
        wrap_js_err(subtle_crypto.decrypt_with_object_and_buffer_source(
            &algorithm,
            &crypto_key,
            &encrypted_metadata,
        ))
        .inspect_err(|e| tracing::error!("Failed to decrypt node metadata (BP): {}", e))?;

    let decrypted_arraybuffer_value =
        wrap_js_err(JsFuture::from(decrypted_arraybuffer_promise).await)
            .inspect_err(|e| tracing::error!("Failed to decrypt node metadata (AP): {}", e))?;

    let decrypted_arraybuffer = wrap_js_err(decrypted_arraybuffer_value.dyn_into())?;
    let decrypted_array = Uint8Array::new(&decrypted_arraybuffer);
    let decrypted_metadata = decrypted_array.to_vec();

    let metadata = serde_json::from_slice(&decrypted_metadata)
        .inspect_err(|e| tracing::error!("Failed to serialize node metadata: {}", e))?;

    Ok(metadata)
}

pub async fn encrypt_metadata(
    metadata: &NodeMetadata,
    metadata_key: &MetadataKey,
) -> Result<EncryptedMetadata, Error> {
    let _guard = debug_span!("utils::encryption::encryptNodeMetadata").entered();

    let decrypted_metadata = serde_json::to_vec(metadata)?;
    let decrypted_metadata_array = Uint8Array::new_from_slice(&decrypted_metadata);

    let subtle_crypto = get_subtle_crypto()?;

    let key = import_key(metadata_key)
        .await
        .inspect_err(|e| tracing::error!("Failed to import key: {}", e))?;

    let iv: IV = random::get_random_iv()?;
    let iv_bytes_array = Uint8Array::new_from_slice(&iv.get());
    let algorithm = AesGcmParams::new(AES_GCM, &iv_bytes_array);

    let encrypted_arraybuffer_promise =
        wrap_js_err(subtle_crypto.encrypt_with_object_and_buffer_source(
            &algorithm,
            &key,
            &decrypted_metadata_array,
        ))
        .inspect_err(|e| tracing::error!("Failed to encrypt node metadata (BP): {}", e))?;

    let encrypted_arraybuffer_value =
        wrap_js_err(JsFuture::from(encrypted_arraybuffer_promise).await)
            .inspect_err(|e| tracing::error!("Failed to encrypt node metadata (AP): {}", e))?;

    let encrypted_arraybuffer = wrap_js_err(encrypted_arraybuffer_value.dyn_into::<ArrayBuffer>())?;

    let encrypted_array = Uint8Array::new(&encrypted_arraybuffer);
    let encrypted_metadata = encrypted_array.to_vec();

    Ok(EncryptedMetadata::new(encrypted_metadata, iv))
}

pub async fn decrypt_node_with_parent(
    parent: &DecryptedNode,
    child: EncryptedNode,
) -> Result<DecryptedNode, Error> {
    let _guard = debug_span!("utils::encryption::decryptNodeWithParent").entered();

    let metadata_child_key = match parent.metadata {
        NodeMetadata::V1(ref metadata_v1) => {
            // find the key in the list of keys
            metadata_v1
                .children_key
                .iter()
                .find(|(id, _)| id.eq(&child.id))
        }
    };

    let key = metadata_child_key
        .ok_or(anyhow!("Key not found"))
        .inspect_err(|e| tracing::error!("Failed to get metadata child key: {}", e))?;
    decrypt_node(child, key.1).await
}

pub async fn decrypt_node_path(start_node: DecryptedNode, path: Vec<EncryptedNode>) -> Result<Vec<DecryptedNode>> {
    let mut decrypted_nodes: Vec<DecryptedNode> = Vec::with_capacity(path.len());
    decrypted_nodes.push(start_node);

    for encrypted_node in path.iter().skip(1) {
        let decrypted_node = decrypt_node_with_parent(
            // last cannot be None, as the vec contains from node
            decrypted_nodes.last().unwrap(),
            encrypted_node.clone(),
        )
            .await
            .inspect_err(|e| tracing::error!("Failed to decrypt node with parent: {}", e))?;

        decrypted_nodes.push(decrypted_node);
    }
    
    Ok(decrypted_nodes)
}

#[cfg(test)]
mod test {
    use crate::model::node::{MetadataV1, NodeMetadata};
    use crate::utils::encryption::generate_aes256_key;
    use crate::utils::encryption::node::{decrypt_metadata, encrypt_metadata};
    use chrono::NaiveDateTime;
    use crabdrive_common::uuid::UUID;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    async fn test_encrypt_decrypt_metadata() {
        let key = generate_aes256_key().await.unwrap();
        let file_key = generate_aes256_key().await.unwrap();

        let example_metadata = NodeMetadata::V1(MetadataV1 {
            name: "hello.txt".to_string(),
            last_modified: NaiveDateTime::default(),
            created: NaiveDateTime::default(),
            size: None,
            mime_type: Some("txt".to_string()),
            file_key: Some(file_key),
            children_key: vec![
                (UUID::random(), generate_aes256_key().await.unwrap()),
                (UUID::random(), generate_aes256_key().await.unwrap()),
                (UUID::random(), generate_aes256_key().await.unwrap()),
            ],
        });

        let encrypted_metadata = encrypt_metadata(&example_metadata, &key)
            .await
            .expect("could not encrypt node");

        let decrypted_metadata = decrypt_metadata(&encrypted_metadata, &key)
            .await
            .expect("could not decrypt node");

        assert_eq!(example_metadata, decrypted_metadata);
    }
}
