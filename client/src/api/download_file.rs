use crate::api::requests::chunk::{GetChunkResponse, get_chunk};
use crate::constants::EMPTY_KEY;
use crate::model::chunk::EncryptedChunk;
use crate::model::node::{DecryptedNode, NodeMetadata};
use crate::utils::encryption::chunk::decrypt_chunk;
use crate::utils::file::combine_chunks;
use crabdrive_common::storage::{ChunkIndex, FileRevision, NodeId};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::js_sys::Uint8Array;
use web_sys::{Blob, Url, window};

pub async fn download_file(node: DecryptedNode) -> Result<(), String> {
    // TODO support chunked downloads in chrom(e/ium)

    let current_revision = node.current_revision;
    if current_revision.is_none() {
        return Err("this node does not have a file associated with it".to_string());
    }
    let current_revision = current_revision.unwrap();

    let mut chunks = Vec::with_capacity(current_revision.chunk_count as usize);

    for i in 1..(current_revision.chunk_count) {
        let decrypted_chunk_result =
            download_chunk_and_decrypt(node.id, &current_revision, i, &"".to_string()).await;
        if let Err(js_error) = decrypted_chunk_result {
            return Err(format!("could not download/decrypt chunk: {:?}", js_error));
        }
        chunks.push(decrypted_chunk_result.unwrap());
    }

    let NodeMetadata::V1(metadata) = node.metadata;
    save_file(combine_chunks(chunks), &metadata.name).await
}

async fn download_chunk_and_decrypt(
    node_id: NodeId,
    revision: &FileRevision,
    i: ChunkIndex,
    token: &String,
) -> Result<Uint8Array, JsValue> {
    let chunk_response = get_chunk(node_id, revision.id, i, token).await?;

    match chunk_response {
        GetChunkResponse::Ok(encrypted_chunk_buffer) => {
            let encrypted_chunk = EncryptedChunk {
                chunk: encrypted_chunk_buffer,
                index: i,
                first_block: i == 1,
                last_block: i == revision.chunk_count,
                iv_prefix: revision.iv,
            };
            //TODO fix key
            let decrypted_chunk = decrypt_chunk(&encrypted_chunk, &EMPTY_KEY).await?;

            Ok(Uint8Array::new(&decrypted_chunk.chunk))
        }
        GetChunkResponse::NotFound => {
            //TODO correct error handling
            Err(JsValue::from(format!("chunk {i} return 404")))
        }
    }
}

async fn save_file(data: Blob, file_name: &str) -> Result<(), String> {
    let url = Url::create_object_url_with_blob(&data).unwrap();
    let window = window().unwrap();
    let document = window.document().unwrap();
    let a = document.create_element("a").unwrap();

    a.set_attribute("href", &url).unwrap();
    a.set_attribute("download", file_name).unwrap();
    a.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
    Url::revoke_object_url(&url).unwrap();

    Ok(())
}
