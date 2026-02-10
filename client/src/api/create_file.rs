use crate::api::requests::chunk::PostChunkResponse;
use crate::constants::CHUNK_SIZE;
use crate::model::chunk::DecryptedChunk;
use crate::model::encryption::{FileKey, MetadataKey, RawEncryptionKey};
use crate::model::node::{DecryptedNode, MetadataV1, NodeMetadata};
use crate::{api, utils};

use crabdrive_common::da;
use crabdrive_common::iv::IV;
use crabdrive_common::payloads::node::request::file::PostCreateFileRequest;
use crabdrive_common::payloads::node::response::file::{
    PostCommitFileResponse, PostCreateFileResponse,
};
use crabdrive_common::storage::{FileRevision, NodeId, RevisionId};

use anyhow::{Context, Result, anyhow};
use chrono::Local;
use wasm_bindgen_futures::js_sys::Uint8Array;
use web_sys::File;

/// Create a new file node. Returns `Err` if called unauthenticated.
///
/// Returns the freshly created node.
pub async fn create_file(
    parent: &mut DecryptedNode,
    file_name: String,
    file: File,
) -> Result<DecryptedNode> {
    let token = utils::auth::get_token()?;

    // The key which is used for encrypting metadata. This will later be stored inside the encrypted
    // metadata of the parent.
    let metadata_encryption_key: MetadataKey = utils::encryption::generate_aes256_key().await?;
    // The key used to encrypt the file chunks. This will be stored in the associated revision.
    let file_encryption_key: FileKey = utils::encryption::generate_aes256_key().await?;

    let file_metadata = NodeMetadata::V1(MetadataV1 {
        name: file_name,
        last_modified: Local::now().naive_local(),
        created: Local::now().naive_local(),
        size: Some(da!(file.size())),
        mime_type: Some(file.type_()),
        file_key: Some(file_encryption_key),
        children_key: vec![],
    });

    let new_node_id = NodeId::random();

    let encrypted_metadata =
        utils::encryption::node::encrypt_metadata(&file_metadata, &metadata_encryption_key)
            .await
            .context("Could not encrypt metadata")?;

    let mut new_parent_metadata = parent.metadata.clone();

    match new_parent_metadata {
        NodeMetadata::V1(ref mut metadata) => metadata
            .children_key
            .push((new_node_id, metadata_encryption_key)),
    }

    let encrypted_parent_metadata =
        utils::encryption::node::encrypt_metadata(&new_parent_metadata, &parent.encryption_key)
            .await
            .context("Could not encrypt metadata")?;

    let chunk_count = (file.size() / CHUNK_SIZE).ceil() as i64;

    let file_iv = utils::encryption::random::get_random_iv()?;
    let request_body = PostCreateFileRequest {
        parent_metadata_version: parent.change_count,
        parent_metadata: encrypted_parent_metadata.clone(),
        node_metadata: encrypted_metadata.clone(),
        file_iv,
        chunk_count,
        node_id: new_node_id,
    };

    let response = api::requests::file::post_create_file(parent.id, request_body, &token)
        .await
        .map_err(|e| anyhow!("Could not create file: {:?}", e))?;

    match response {
        PostCreateFileResponse::Created(new_file) => {
            parent.metadata = new_parent_metadata;
            parent.change_count += 2; // First update the parent metadata, then insert the new node

            let file_revision = new_file.current_revision;

            if file_revision.is_none() {
                return Err(anyhow!(
                    "The server did not create a file revision when creating the file"
                ));
            }

            let file_revision = file_revision.unwrap();

            // if this fails the server is lying to us
            if file_revision.iv != file_iv {
                return Err(anyhow!("The server is lying to us"));
            }

            //TODO test this
            upload_file(
                file,
                file_encryption_key,
                &file_revision,
                new_node_id,
                &token,
            )
            .await
        }
        PostCreateFileResponse::NotFound => Err(anyhow!(
            "No such node: {}. Check if you have permission to access it",
            parent.id
        )),
        PostCreateFileResponse::BadRequest => Err(anyhow!("Bad request")),
        PostCreateFileResponse::Conflict => Err(anyhow!("Bad request")),
    }
}

async fn upload_file(
    file: File,
    key: RawEncryptionKey,
    revision: &FileRevision,
    node_id: NodeId,
    token: &String,
) -> Result<DecryptedNode> {
    //TODO test this
    utils::file::load_file_by_chunk(file, |chunk| {
        // this does not clone the actual arraybuffer, just the ref to it
        let chunk = chunk.clone();
        async move {
            encrypt_and_upload_chunk(&chunk, revision.iv, &key, node_id, revision.id, token).await
        }
    })
    .await?;

    let response = api::requests::file::post_commit_file(node_id, revision.id, token).await?;

    match response {
        PostCommitFileResponse::Ok(encrypted_node) => {
            let decrypted_node = utils::encryption::node::decrypt_node(encrypted_node, key).await?;
            Ok(decrypted_node)
        }
        PostCommitFileResponse::BadRequest(err) => {
            Err(anyhow!("Server returned bad request: {:?}", err))
        }
        PostCommitFileResponse::NotFound => Err(anyhow!("no such node: {}", node_id)),
    }
}

async fn encrypt_and_upload_chunk(
    chunk: &DecryptedChunk,
    iv_prefix: IV,
    key: &RawEncryptionKey,
    node_id: NodeId,
    revision_id: RevisionId,
    token: &String,
) -> Result<()> {
    let encrypted_chunk = utils::encryption::chunk::encrypt_chunk(chunk, key, iv_prefix).await?;
    let request_body = Uint8Array::new(&encrypted_chunk.chunk);
    let response =
        api::requests::chunk::post_chunk(node_id, revision_id, chunk.index, request_body, token)
            .await?;

    //TODO error handling
    match response {
        PostChunkResponse::Created => Ok(()),
        PostChunkResponse::OutOfStorage => Err(anyhow!("You have exceeded your quota")),
        _ => Err(anyhow!(
            "Unexpected error while uploading chunk: {:?}",
            response
        )),
    }
}
