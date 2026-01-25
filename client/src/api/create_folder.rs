use crabdrive_common::payloads::node::request::folder::PostCreateFolderRequest;
use crabdrive_common::payloads::node::response::folder::PostCreateFolderResponse;
use crabdrive_common::storage::NodeId;
use crate::api::requests::folder::post_create_folder;
use crate::constants::EMPTY_KEY;
use crate::model::node::{DecryptedNode, MetadataV1, NodeMetadata};
use crate::utils::encryption::node::{decrypt_node, encrypt_metadata};

pub async fn create_folder(
    parent: &mut DecryptedNode,
    folder_name: String,
) -> Result<DecryptedNode, String> {
    let folder_metadata = NodeMetadata::V1(MetadataV1 {
        name: folder_name,
        last_modified: Default::default(),
        created: Default::default(),
        size: None,
        mime_type: None,
        file_key: None,
        children_key: vec![],
    });

    //TODO actually generate encryption key
    //let new_encryption_key = get_random_encryption_key();

    let new_node_encryption_key = EMPTY_KEY;
    let new_node_id = NodeId::random();

    let encrypted_metadata = encrypt_metadata(&folder_metadata, &new_node_encryption_key)
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

    let request_body = PostCreateFolderRequest {
        parent_metadata_iv: *encrypted_parent_metadata.iv(),
        parent_metadata_version: parent.change_count,
        parent_metadata: encrypted_parent_metadata.metadata().clone(),
        node_metadata_iv: *encrypted_metadata.iv(),
        node_metadata: encrypted_metadata.metadata().clone(),
        node_id: new_node_id,
    };

    let response = post_create_folder(parent.id, request_body, &"".to_string())
        .await
        .expect("failed to post create folder");

    match response {
        PostCreateFolderResponse::Created(new_folder) => {
            parent.metadata = new_parent_metadata;
            parent.change_count += 1;

            let decrypted_node = decrypt_node(new_folder, new_node_encryption_key)
                .await
                .unwrap();

            Ok(decrypted_node)
        }
        PostCreateFolderResponse::NotFound => Err(format!(
            "no such node: {}. Check if you have permission to access it",
            parent.id
        )),
        PostCreateFolderResponse::BadRequest => Err("bad request".to_string()),
        PostCreateFolderResponse::Conflict => Err("Please try again".to_string()),
    }
}