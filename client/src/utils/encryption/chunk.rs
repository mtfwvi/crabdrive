use crate::api::requests::chunk::{post_chunk, PostChunkResponse};
use crate::constants::AES_GCM;
use crate::model::encryption::EncryptionKey;
use crate::utils::encryption;
use crabdrive_common::iv::IV;
use crabdrive_common::storage::{NodeId, RevisionId};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_futures::js_sys::{ArrayBuffer, Uint8Array};
use web_sys::AesGcmParams;
use crate::model::chunk::{DecryptedChunk, EncryptedChunk};

/// The byte in the authenticated data that indicates the start and end of files to prevent
/// truncating by the server
fn get_additional_byte(first_chunk: bool, last_chunk: bool) -> u8 {
    match (first_chunk, last_chunk) {
        (true, false) => 1,  // start chunk
        (false, true) => 2,  // end chunk
        (true, true) => 3,   // only chunk
        (false, false) => 4, // in between chunk
    }
}

pub async fn decrypt_chunk(
    chunk: &EncryptedChunk,
    key: &EncryptionKey,
) -> Result<DecryptedChunk, JsValue> {
    let subtle_crypto = encryption::get_subtle_crypto();
    let key = encryption::get_key_from_bytes(key).await;

    let additional_byte = get_additional_byte(chunk.first_block, chunk.last_block);
    let additional_data = Uint8Array::new_from_slice(&[additional_byte]);

    let iv_bytes = chunk.iv_prefix.prefix_with_u32(chunk.index as u32);
    let iv_bytes_array = Uint8Array::new_from_slice(&iv_bytes);
    let algorithm = AesGcmParams::new(AES_GCM, &iv_bytes_array);
    algorithm.set_additional_data(&additional_data);

    let decrypted_arraybuffer_promise =
        subtle_crypto.decrypt_with_object_and_buffer_source(&algorithm, &key, &chunk.chunk)?;
    let decrypted_arraybuffer: ArrayBuffer = JsFuture::from(decrypted_arraybuffer_promise)
        .await?
        .dyn_into()?;

    Ok(DecryptedChunk {
        chunk: decrypted_arraybuffer,
        index: chunk.index,
        first_block: chunk.first_block,
        last_block: chunk.last_block,
    })
}

pub async fn encrypt_chunk(
    chunk: &DecryptedChunk,
    key: &EncryptionKey,
    iv_prefix: IV,
) -> Result<EncryptedChunk, JsValue> {
    let subtle_crypto = encryption::get_subtle_crypto();
    let key = encryption::get_key_from_bytes(key).await;

    let additional_byte = get_additional_byte(chunk.first_block, chunk.last_block);
    let additional_data = Uint8Array::new_from_slice(&[additional_byte]);
    let iv_bytes = iv_prefix.prefix_with_u32(chunk.index as u32);
    let iv_bytes_array = Uint8Array::new_from_slice(&iv_bytes);
    let algorithm = AesGcmParams::new(AES_GCM, &iv_bytes_array);
    algorithm.set_additional_data(&additional_data);
    let encrypted_arraybuffer_promise =
        subtle_crypto.encrypt_with_object_and_buffer_source(&algorithm, &key, &chunk.chunk)?;
    let encrypted_arraybuffer: ArrayBuffer = JsFuture::from(encrypted_arraybuffer_promise)
        .await?
        .dyn_into()?;

    Ok(EncryptedChunk {
        chunk: Uint8Array::new(&encrypted_arraybuffer),
        index: chunk.index,
        first_block: chunk.first_block,
        last_block: chunk.last_block,
        iv_prefix,
    })
}

pub async fn encrypt_and_upload_chunk(
    chunk: &DecryptedChunk,
    iv_prefix: IV,
    key: &EncryptionKey,
    node_id: NodeId,
    revision_id: RevisionId,
) -> Result<(), JsValue> {
    let encrypted_chunk = encrypt_chunk(chunk, key, iv_prefix)
        .await
        .expect("failed to encrypt chunk");

    let request_body = Uint8Array::new(&encrypted_chunk.chunk);

    let response = post_chunk(
        node_id,
        revision_id,
        chunk.index,
        request_body,
        &"".to_string(),
    )
    .await?;

    //TODO error handling
    match response {
        PostChunkResponse::Created => Ok(()),
        PostChunkResponse::NotFound => {
            panic!("404 when uploading chunk")
        }
        PostChunkResponse::BadRequest => {
            panic!("400 when uploading chunk")
        }
        PostChunkResponse::Conflict => {
            panic!("409 when uploading chunk")
        }
        PostChunkResponse::OutOfStorage => {
            panic!("413 when uploading chunk")
        }
    }
}
