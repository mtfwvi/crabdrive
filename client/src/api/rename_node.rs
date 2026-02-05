use crate::api::requests::node::patch_node;
use crate::constants::EMPTY_KEY;
use crate::model::node::{DecryptedNode, MetadataV1, NodeMetadata};
use crate::utils::encryption::node::encrypt_metadata;
use crabdrive_common::payloads::node::request::node::PatchNodeRequest;
use crabdrive_common::payloads::node::response::node::PatchNodeResponse;

pub async fn rename_node(node: DecryptedNode, new_name: String) -> Result<(), String> {
    let new_node_encryption_key = EMPTY_KEY;

    let NodeMetadata::V1(old_metadata) = node.metadata;

    let file_metadata = NodeMetadata::V1(MetadataV1 {
        name: new_name,
        ..old_metadata
    });

    let encrypted_metadata_result =
        encrypt_metadata(&file_metadata, &new_node_encryption_key).await;

    if let Err(js_error) = encrypted_metadata_result {
        return Err(format!("could not encrypt metadata: {:?}", js_error));
    }
    let encrypted_metadata = encrypted_metadata_result.unwrap();

    let request_body = PatchNodeRequest {
        node_metadata: encrypted_metadata.clone(),
        node_change_count: node.change_count,
    };

    let response = patch_node(node.id, request_body, &"".to_string()).await;

    if let Err(js_error) = response {
        return Err(format!("could not patch node: {:?}", js_error));
    }

    let response = response.unwrap();

    match response {
        PatchNodeResponse::Ok(_) => Ok(()),
        PatchNodeResponse::NotFound => Err(format!(
            "no such node: {}. Check if you have permission to access it",
            node.id
        )),
        PatchNodeResponse::Conflict => Err("Please try again".to_string()),
    }
}
