use crate::api::requests::chunk::{get_chunk, GetChunkResponse};
use crate::constants::EMPTY_KEY;
use crate::model::chunk::EncryptedChunk;
use crate::model::node::{DecryptedNode, NodeMetadata};
use crate::utils::browser::get_document;
use crate::utils::encryption::chunk::decrypt_chunk;
use crate::utils::error::wrap_js_err;
use crate::utils::file::combine_chunks;
use anyhow::{anyhow, Result};
use crabdrive_common::storage::{ChunkIndex, FileRevision, NodeId};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::js_sys::Uint8Array;
use web_sys::{Blob, Url};

pub async fn download_file(node: DecryptedNode) -> Result<()> {
    // TODO support chunked downloads in chrom(e/ium)

    let current_revision = node.current_revision;

    if current_revision.is_none() {
        return Err(anyhow!("this node does not have a file associated with it"));
    }

    let current_revision = current_revision.unwrap();

    let mut chunks = Vec::with_capacity(current_revision.chunk_count as usize);

    for i in 1..=current_revision.chunk_count {
        let decrypted_chunk_result =
            download_chunk_and_decrypt(node.id, &current_revision, i, &"".to_string()).await?;
        chunks.push(decrypted_chunk_result);
    }

    let NodeMetadata::V1(metadata) = node.metadata;
    save_file(combine_chunks(chunks)?, &metadata.name).await
}

async fn download_chunk_and_decrypt(
    node_id: NodeId,
    revision: &FileRevision,
    i: ChunkIndex,
    token: &String,
) -> Result<Uint8Array> {
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
            Err(anyhow!("chunk {i} returned 404"))
        }
    }
}

async fn save_file(data: Blob, file_name: &str) -> Result<()> {
    let url = wrap_js_err(Url::create_object_url_with_blob(&data))?;
    let document = get_document()?;
    let a = wrap_js_err(document.create_element("a"))?;

    wrap_js_err(a.set_attribute("href", &url))?;
    wrap_js_err(a.set_attribute("download", file_name))?;
    a.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
    wrap_js_err(Url::revoke_object_url(&url))?;

    Ok(())
}
