use crate::constants::AES_GCM;
use crate::model::chunk::{DecryptedChunk, EncryptedChunk};
use crate::model::encryption::EncryptionKey;
use crate::utils::browser::get_subtle_crypto;
use crate::utils::encryption;
use crate::utils::error::{future_from_js_promise, wrap_js_err};
use anyhow::Result;
use crabdrive_common::iv::IV;
use crabdrive_common::storage::ChunkIndex;
use wasm_bindgen_futures::js_sys::{ArrayBuffer, Uint8Array};
use web_sys::AesGcmParams;

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

pub async fn decrypt_chunk(chunk: &EncryptedChunk, key: &EncryptionKey) -> Result<DecryptedChunk> {
    let subtle_crypto = get_subtle_crypto()?;
    let key = encryption::get_key_from_bytes(key).await?;

    let encryption_params = get_encryption_params(
        chunk.first_block,
        chunk.last_block,
        chunk.index,
        chunk.iv_prefix,
    );

    let decrypted_arraybuffer_promise = wrap_js_err(
        subtle_crypto.decrypt_with_object_and_buffer_source(&encryption_params, &key, &chunk.chunk),
    )?;

    let decrypted_arraybuffer: ArrayBuffer =
        future_from_js_promise(decrypted_arraybuffer_promise).await?;

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
) -> Result<EncryptedChunk> {
    let subtle_crypto = get_subtle_crypto()?;
    let key = encryption::get_key_from_bytes(key).await?;

    let encryption_params =
        get_encryption_params(chunk.first_block, chunk.last_block, chunk.index, iv_prefix);

    let encrypted_arraybuffer_promise = wrap_js_err(
        subtle_crypto.encrypt_with_object_and_buffer_source(&encryption_params, &key, &chunk.chunk),
    )?;

    let encrypted_arraybuffer: ArrayBuffer =
        future_from_js_promise(encrypted_arraybuffer_promise).await?;

    Ok(EncryptedChunk {
        chunk: Uint8Array::new(&encrypted_arraybuffer),
        index: chunk.index,
        first_block: chunk.first_block,
        last_block: chunk.last_block,
        iv_prefix,
    })
}

fn get_encryption_params(
    first_chunk: bool,
    last_chunk: bool,
    index: ChunkIndex,
    iv_prefix: IV,
) -> AesGcmParams {
    let additional_byte = get_additional_byte(first_chunk, last_chunk);
    let additional_data = Uint8Array::new_from_slice(&[additional_byte]);
    let iv_bytes = iv_prefix.prefix_with_u32(index as u32);
    let iv_bytes_array = Uint8Array::new_from_slice(&iv_bytes);
    let algorithm = AesGcmParams::new(AES_GCM, &iv_bytes_array);
    algorithm.set_additional_data(&additional_data);

    algorithm
}

#[cfg(test)]
mod test {
    use crate::constants::EMPTY_KEY;
    use crate::model::chunk::DecryptedChunk;
    use crate::utils::encryption::chunk::{decrypt_chunk, encrypt_chunk};
    use crabdrive_common::iv::IV;
    use wasm_bindgen_futures::js_sys::Uint8Array;
    use wasm_bindgen_test::wasm_bindgen_test;

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
    async fn test_detect_chunk_removal() {
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

        // server tries to truncate the file by removing the first block
        encrypted_chunk.index = 1;
        encrypted_chunk.first_block = true;

        let decrypted_chunk = decrypt_chunk(&encrypted_chunk, &EMPTY_KEY).await;

        assert!(decrypted_chunk.is_err())
    }
}
