use crate::constants::AES_GCM;
use crate::model::encryption::EncryptionKey;
use crate::model::node::{DecryptedNode, NodeMetadata};
use crate::utils::browser::get_subtle_crypto;
use crate::utils::encryption::{get_key_from_bytes, random};
use crate::utils::error::wrap_js_err;
use anyhow::{Error, anyhow};
use crabdrive_common::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::iv::IV;
use crabdrive_common::storage::EncryptedNode;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_futures::js_sys::{ArrayBuffer, Uint8Array};
use web_sys::AesGcmParams;

pub async fn decrypt_node(node: EncryptedNode, key: EncryptionKey) -> Result<DecryptedNode, Error> {
    let decrypted_metadata = decrypt_metadata(&node.encrypted_metadata, &key).await?;

    let decrypted_node = DecryptedNode {
        id: node.id,
        change_count: node.change_count,
        parent_id: node.parent_id,
        owner_id: node.owner_id,
        deleted_on: node.deleted_on,
        node_type: node.node_type,
        current_revision: node.current_revision,
        metadata: decrypted_metadata,
        encryption_key: key,
    };

    Ok(decrypted_node)
}

pub async fn decrypt_metadata(
    metadata: &EncryptedMetadata,
    key: &EncryptionKey,
) -> Result<NodeMetadata, Error> {
    let encrypted_metadata = Uint8Array::new_from_slice(metadata.metadata());

    let subtle_crypto = get_subtle_crypto()?;
    let crypto_key = get_key_from_bytes(key).await?;

    let iv_bytes = metadata.iv();
    let iv_bytes_array = Uint8Array::new_from_slice(&iv_bytes.get());
    let algorithm = AesGcmParams::new(AES_GCM, &iv_bytes_array);

    let decrypted_arraybuffer_promise =
        wrap_js_err(subtle_crypto.decrypt_with_object_and_buffer_source(
            &algorithm,
            &crypto_key,
            &encrypted_metadata,
        ))?;
    let decrypted_arraybuffer_value =
        wrap_js_err(JsFuture::from(decrypted_arraybuffer_promise).await)?;

    let decrypted_arraybuffer = wrap_js_err(decrypted_arraybuffer_value.dyn_into())?;
    let decrypted_array = Uint8Array::new(&decrypted_arraybuffer);
    let decrypted_metadata = decrypted_array.to_vec();

    let metadata = serde_json::from_slice(&decrypted_metadata)?;

    Ok(metadata)
}

pub async fn encrypt_metadata(
    metadata: &NodeMetadata,
    key: &EncryptionKey,
) -> Result<EncryptedMetadata, Error> {
    let decrypted_metadata = serde_json::to_vec(metadata)?;
    let decrypted_metadata_array = Uint8Array::new_from_slice(&decrypted_metadata);

    let iv: IV = random::get_random_iv()?;

    let subtle_crypto = get_subtle_crypto()?;
    let key = get_key_from_bytes(key).await?;

    let iv_bytes_array = Uint8Array::new_from_slice(&iv.get());
    let algorithm = AesGcmParams::new(AES_GCM, &iv_bytes_array);

    let encrypted_arraybuffer_promise =
        wrap_js_err(subtle_crypto.encrypt_with_object_and_buffer_source(
            &algorithm,
            &key,
            &decrypted_metadata_array,
        ))?;
    let encrypted_arraybuffer_value =
        wrap_js_err(JsFuture::from(encrypted_arraybuffer_promise).await)?;

    let encrypted_arraybuffer = wrap_js_err(encrypted_arraybuffer_value.dyn_into::<ArrayBuffer>())?;

    let encrypted_array = Uint8Array::new(&encrypted_arraybuffer);
    let encrypted_metadata = encrypted_array.to_vec();

    Ok(EncryptedMetadata::new(encrypted_metadata, iv))
}

pub async fn decrypt_node_with_parent(
    parent: &DecryptedNode,
    child: EncryptedNode,
) -> Result<DecryptedNode, Error> {
    let key = match parent.metadata {
        NodeMetadata::V1(ref metadata_v1) => {
            // find the key in the list of keys
            metadata_v1
                .children_key
                .iter()
                .find(|(id, _)| id.eq(&child.id))
        }
    };

    if key.is_none() {
        return Err(anyhow!("key not found"));
    }
    let key = key.unwrap();
    decrypt_node(child, key.1).await
}

#[cfg(test)]
mod test {
    use crate::constants::EMPTY_KEY;
    use crate::model::node::{MetadataV1, NodeMetadata};
    use crate::utils::encryption::node::{decrypt_metadata, encrypt_metadata};
    use chrono::NaiveDateTime;
    use wasm_bindgen_test::wasm_bindgen_test;

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
