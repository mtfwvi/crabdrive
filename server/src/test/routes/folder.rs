use crate::test::utils::TestContext;

use crabdrive_common::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::payloads::node::{
    request::folder::*,
    response::folder::*,
};
use crabdrive_common::routes;
use crabdrive_common::storage::NodeType;
use crabdrive_common::uuid::UUID;

use axum::http::StatusCode;
use pretty_assertions::assert_eq;

#[tokio::test]
pub async fn test_create_folder() {
    let ctx = TestContext::new(1).await;

    let user1 = ctx.get_user(0);
    let user1_id = user1.id;
    let user1_name = user1.username.clone();

    let id = UUID::random();
    let metadata = EncryptedMetadata::random();

    let create_folder_body = PostCreateFolderRequest {
        parent_metadata_version: 0,
        parent_metadata: EncryptedMetadata::random(),
        node_metadata: metadata.clone(),
        node_id: id,
    };

    let request = user1
        .post(routes::node::folder::create(user1.get_root()))
        .json(&create_folder_body)
        .await;

    assert_eq!(request.status_code(), StatusCode::CREATED);
    assert!(ctx.node.get_node(id).expect("Failed to get node").is_some());

    let node = match request.json::<PostCreateFolderResponse>() {
        PostCreateFolderResponse::Created(encrypted_node) => encrypted_node,
        PostCreateFolderResponse::NotFound => panic!("Invalid Status code"),
        PostCreateFolderResponse::BadRequest => panic!("Invalid Status code"),
        PostCreateFolderResponse::Conflict => panic!("Invalid Status code"),
    };

    assert!(node.current_revision.is_none());
    assert!(node.deleted_on.is_none());
    assert_eq!(node.node_type, NodeType::Folder);
    assert_eq!(node.encrypted_metadata, metadata);
    assert_eq!(node.has_access, vec![(user1_id, user1_name)]);
}

#[tokio::test]
pub async fn test_create_folder_in_invalid_parent() {
    let ctx = TestContext::new(1).await;

    let user1 = ctx.get_user(0);

    let id = UUID::random();

    let create_folder_body = PostCreateFolderRequest {
        parent_metadata_version: 999,
        parent_metadata: EncryptedMetadata::random(),
        node_metadata: EncryptedMetadata::random(),
        node_id: id,
    };

    let request = user1
        .post(routes::node::folder::create(UUID::random()))
        .json(&create_folder_body)
        .await;

    assert_eq!(request.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
pub async fn test_create_folder_with_parent_metadata_mismatch() {
    let ctx = TestContext::new(1).await;

    let user1 = ctx.get_user(0);

    let id = UUID::random();

    let create_folder_body = PostCreateFolderRequest {
        parent_metadata_version: 999,
        parent_metadata: EncryptedMetadata::random(),
        node_metadata: EncryptedMetadata::random(),
        node_id: id,
    };

    let request = user1
        .post(routes::node::folder::create(user1.get_root()))
        .json(&create_folder_body)
        .await;

    assert_eq!(request.status_code(), StatusCode::CONFLICT);
}

#[tokio::test]
pub async fn test_create_folder_in_file() {
    let ctx = TestContext::new(1).await;

    let user1 = ctx.get_user(0);
    let file1 = user1.generate_random_file().await;

    let id = UUID::random();

    let create_folder_body = PostCreateFolderRequest {
        parent_metadata_version: 999,
        parent_metadata: EncryptedMetadata::random(),
        node_metadata: EncryptedMetadata::random(),
        node_id: id,
    };

    let request = user1
        .post(routes::node::folder::create(file1.id))
        .json(&create_folder_body)
        .await;

    assert_eq!(request.status_code(), StatusCode::BAD_REQUEST);
}
