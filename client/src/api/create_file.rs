use crate::api::requests::chunk::{PostChunkResponse, post_chunk};
use crate::api::requests::file::{post_commit_file, post_create_file};
use crate::constants::{CHUNK_SIZE, EMPTY_KEY};
use crate::model::chunk::DecryptedChunk;
use crate::model::encryption::EncryptionKey;
use crate::model::node::{DecryptedNode, MetadataV1, NodeMetadata};
use crate::utils::encryption::chunk;
use crate::utils::encryption::node::{decrypt_node, encrypt_metadata};
use crate::utils::encryption::random::get_random_iv;
use crate::utils::file::load_file_by_chunk;
use crabdrive_common::iv::IV;
use crabdrive_common::payloads::node::request::file::PostCreateFileRequest;
use crabdrive_common::payloads::node::response::file::{
    PostCommitFileResponse, PostCreateFileResponse,
};
use crabdrive_common::storage::{FileRevision, NodeId, RevisionId};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::js_sys::Uint8Array;
use web_sys::File;

pub async fn create_file(
    parent: &mut DecryptedNode,
    file_name: String,
    file: File,
) -> Result<DecryptedNode, String> {
    //TODO actually generate encryption keys
    //let new_encryption_key = get_random_encryption_key();

    let new_node_encryption_key = EMPTY_KEY;

    let file_encryption_key = EMPTY_KEY;

    let file_metadata = NodeMetadata::V1(MetadataV1 {
        name: file_name,
        last_modified: Default::default(),
        created: Default::default(),
        size: None,
        mime_type: None,
        file_key: Some(file_encryption_key),
        children_key: vec![],
    });

    let new_node_id = NodeId::random();

    let encrypted_metadata = encrypt_metadata(&file_metadata, &new_node_encryption_key)
        .await
        .unwrap();

    let mut new_parent_metadata = parent.metadata.clone();

    match new_parent_metadata {
        NodeMetadata::V1(ref mut metadata) => metadata
            .children_key
            .push((new_node_id, new_node_encryption_key)),
    }

    let encrypted_parent_metadata = encrypt_metadata(&new_parent_metadata, &parent.encryption_key)
        .await
        .unwrap();

    let chunk_count = (file.size() / CHUNK_SIZE).ceil() as u64;

    let file_iv = get_random_iv();
    let request_body = PostCreateFileRequest {
        parent_metadata_iv: *encrypted_parent_metadata.iv(),
        parent_metadata_version: parent.change_count,
        parent_metadata: encrypted_parent_metadata.metadata().clone(),
        node_metadata_iv: *encrypted_metadata.iv(),
        node_metadata: encrypted_metadata.metadata().clone(),
        file_iv,
        chunk_count,
        node_id: new_node_id,
    };

    let response = post_create_file(parent.id, request_body, &"".to_string()).await;

    if let Err(js_error) = response {
        return Err(format!("could not create file: {:?}", js_error));
    }

    let response = response.unwrap();

    match response {
        PostCreateFileResponse::Created(new_file) => {
            parent.metadata = new_parent_metadata;
            parent.change_count += 1;

            let file_revision = new_file
                .current_revision
                .expect("The server did not create a file revision when creating the file");

            // if this fails the server is lying to us
            assert_eq!(file_revision.iv, file_iv);

            //TODO test this
            upload_file(
                file,
                file_encryption_key,
                &file_revision,
                new_node_id,
                &"".to_string(),
            )
            .await
        }
        PostCreateFileResponse::NotFound => Err(format!(
            "no such node: {}. Check if you have permission to access it",
            parent.id
        )),
        PostCreateFileResponse::BadRequest => Err("bad request".to_string()),
        PostCreateFileResponse::Conflict => Err("Please try again".to_string()),
    }
}

async fn upload_file(
    file: File,
    key: EncryptionKey,
    revision: &FileRevision,
    node_id: NodeId,
    token: &String,
) -> Result<DecryptedNode, String> {
    //TODO test this
    let result = load_file_by_chunk(file, |chunk| {
        // this does not clone the actual arraybuffer, just the ref to it
        let chunk = chunk.clone();
        async move {
            encrypt_and_upload_chunk(&chunk, revision.iv, &key, node_id, revision.id, token).await
        }
    })
    .await;

    if let Err(js_error) = result {
        return Err(format!("could not upload chunks: {:?}", js_error));
    }

    let response = post_commit_file(node_id, revision.id, token).await;

    if let Err(ref js_error) = response {
        return Err(format!("could not commit file: {:?}", js_error));
    };

    let response = response.unwrap();
    match response {
        PostCommitFileResponse::Ok(encrypted_node) => {
            let decrypted_node = decrypt_node(encrypted_node, key).await.unwrap();
            Ok(decrypted_node)
        }
        PostCommitFileResponse::BadRequest(missing_chunks) => {
            Err(format!("missing chunks: {:?}", missing_chunks))
        }
        PostCommitFileResponse::NotFound => Err(format!("no such node: {}", node_id)),
    }
}

async fn encrypt_and_upload_chunk(
    chunk: &DecryptedChunk,
    iv_prefix: IV,
    key: &EncryptionKey,
    node_id: NodeId,
    revision_id: RevisionId,
    token: &String,
) -> Result<(), JsValue> {
    let encrypted_chunk = chunk::encrypt_chunk(chunk, key, iv_prefix)
        .await
        .expect("failed to encrypt chunk");

    let request_body = Uint8Array::new(&encrypted_chunk.chunk);

    let response = post_chunk(node_id, revision_id, chunk.index, request_body, token).await?;

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
