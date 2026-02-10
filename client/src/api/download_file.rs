use crate::api::requests::chunk::GetChunkResponse;
use crate::model::chunk::EncryptedChunk;
use crate::model::encryption::FileKey;
use crate::model::node::{DecryptedNode, NodeMetadata};
use crate::utils::error::wrap_js_err;
use crate::{api, utils};
use anyhow::{Result, anyhow};
use crabdrive_common::storage::{ChunkIndex, FileRevision, NodeId};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::js_sys::Uint8Array;
use web_sys::{Blob, Url};

pub async fn download_file(node: DecryptedNode) -> Result<()> {
    // TODO support chunked downloads in chrom(e/ium)
    let token = utils::auth::get_token()?;

    let file_key = match &node.metadata {
        NodeMetadata::V1(n) => n.file_key,
    }
    .ok_or(anyhow!("Cannot download folders/symlinks"))?;

    let current_revision = node
        .current_revision
        .ok_or(anyhow!("This node does not have any file contents"))?;

    let mut chunks = Vec::with_capacity(current_revision.chunk_count as usize);

    for i in 1..=current_revision.chunk_count {
        let decrypted_chunk_result =
            download_chunk_and_decrypt(node.id, &current_revision, &file_key, i, &token).await?;
        chunks.push(decrypted_chunk_result);
    }

    let name = match &node.metadata {
        NodeMetadata::V1(n) => &n.name,
    };

    save_file(utils::file::combine_chunks(chunks)?, name).await
}

async fn download_chunk_and_decrypt(
    node_id: NodeId,
    revision: &FileRevision,
    key: &FileKey,
    index: ChunkIndex,
    token: &String,
) -> Result<Uint8Array> {
    let chunk_response =
        api::requests::chunk::get_chunk(node_id, revision.id, index, token).await?;

    match chunk_response {
        GetChunkResponse::Ok(encrypted_chunk_buffer) => {
            let encrypted_chunk = EncryptedChunk {
                chunk: encrypted_chunk_buffer,
                index,
                first_block: index == 1,
                last_block: index == revision.chunk_count,
                iv_prefix: revision.iv,
            };

            let decrypted_chunk =
                utils::encryption::chunk::decrypt_chunk(&encrypted_chunk, key).await?;

            Ok(Uint8Array::new(&decrypted_chunk.chunk))
        }
        GetChunkResponse::NotFound => {
            //TODO correct error handling
            Err(anyhow!("Chunk {index} does not exist"))
        }
    }
}

async fn save_file(data: Blob, file_name: &str) -> Result<()> {
    let url = wrap_js_err(Url::create_object_url_with_blob(&data))?;
    let document = utils::browser::get_document()?;
    let a = wrap_js_err(document.create_element("a"))?;

    wrap_js_err(a.set_attribute("href", &url))?;
    wrap_js_err(a.set_attribute("download", file_name))?;
    a.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
    wrap_js_err(Url::revoke_object_url(&url))?;

    Ok(())
}
