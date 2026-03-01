use crate::test::utils::TestContext;

use crabdrive_common::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::payloads::node::{request::node::*, response::node::*};
use crabdrive_common::routes;
use crabdrive_common::uuid::UUID;

use axum::http::StatusCode;
use pretty_assertions::assert_eq;

#[tokio::test]
pub async fn test_get_node() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let folder = user.generate_random_folder().await;

    let response = user.get(routes::node::by_id(folder.id)).await;
    assert_eq!(response.status_code(), StatusCode::OK);

    let GetNodeResponse::Ok(node) = response.json::<GetNodeResponse>() else {
        panic!("Expected Ok with node data");
    };

    assert_eq!(node.id, folder.id);
}

#[tokio::test]
pub async fn test_get_invalid_node() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let response = user.get(routes::node::by_id(UUID::random())).await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
}

// patch node

#[tokio::test]
pub async fn test_patch_node() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let folder = user.generate_random_folder().await;
    let node_entity = user.fetch_node_from_db(folder.id).unwrap();

    let payload = PatchNodeRequest {
        node_metadata: EncryptedMetadata::random(),
        node_change_count: node_entity.metadata_change_counter,
    };

    let response = user
        .patch(routes::node::by_id(folder.id))
        .json(&payload)
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
pub async fn test_patch_invalid_node() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let payload = PatchNodeRequest {
        node_metadata: EncryptedMetadata::random(),
        node_change_count: 0,
    };

    let response = user
        .patch(routes::node::by_id(UUID::random()))
        .json(&payload)
        .await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
pub async fn test_patch_node_metadata_mismatch() {
    let ctx = TestContext::new(1).await;

    let user1 = ctx.get_user(0);

    let folder = user1.generate_random_folder().await;

    let payload = PatchNodeRequest {
        node_metadata: EncryptedMetadata::random(),
        node_change_count: 0,
    };

    let response = user1
        .patch(routes::node::by_id(folder.id))
        .json(&payload)
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let payload = PatchNodeRequest {
        node_metadata: EncryptedMetadata::random(),
        node_change_count: 0,
    };

    let response = user1
        .patch(routes::node::by_id(folder.id))
        .json(&payload)
        .await;

    assert_eq!(response.status_code(), StatusCode::CONFLICT);
}

// delete node

#[tokio::test]
pub async fn test_delete_node() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let folder = user.generate_random_folder().await;

    // Move node to trash
    let payload = PostMoveNodeRequest {
        from_node_change_counter: 1,
        from_node_metadata: EncryptedMetadata::random(),
        to_node_change_counter: 0,
        to_node_metadata: EncryptedMetadata::random(),
        to_node_id: user.get_trash(),
    };

    let response = user
        .post(routes::node::move_to_trash(folder.id))
        .json(&payload)
        .await;

    response.assert_status_ok();

    let payload = DeleteNodeRequest {
        parent_change_count: 1,
        parent_node_metadata: EncryptedMetadata::random(),
    };

    let response = user
        .delete(routes::node::by_id(folder.id))
        .json(&payload)
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let DeleteNodeResponse::Ok = response.json() else {
        panic!("Expected Ok");
    };

    let get_resp = user.get(routes::node::by_id(folder.id)).await;
    assert_eq!(get_resp.status_code(), StatusCode::NOT_FOUND);
}

// move node

#[tokio::test]
pub async fn test_move_file_into_folder() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let source_folder = user.generate_random_folder().await;
    let target_folder = user.generate_random_folder().await;
    let file = user.generate_file_in(source_folder.id).await;

    let payload = PostMoveNodeRequest {
        to_node_id: target_folder.id,
        from_node_metadata: EncryptedMetadata::random(),
        to_node_metadata: EncryptedMetadata::random(),
        from_node_change_counter: 0,
        to_node_change_counter: 0,
    };

    let response = user
        .post(routes::node::move_to(file.id))
        .json(&payload)
        .await;

    assert_eq!(response.status_code(), StatusCode::CONFLICT);
}

#[tokio::test]
pub async fn test_move_file_into_file() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let file1 = user.generate_random_file().await;
    let file2 = user.generate_random_file().await;

    let payload = PostMoveNodeRequest {
        to_node_id: file2.id,
        from_node_metadata: EncryptedMetadata::random(),
        to_node_metadata: EncryptedMetadata::random(),
        from_node_change_counter: 0,
        to_node_change_counter: 0,
    };

    let request = user
        .post(routes::node::move_to(file1.id))
        .json(&payload)
        .await;

    request.assert_status_bad_request();
}

#[tokio::test]
pub async fn test_move_folder_into_folder() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let folder1 = user.generate_random_folder().await;
    let folder2 = user.generate_random_folder().await;

    let payload = PostMoveNodeRequest {
        to_node_id: folder2.id,
        from_node_metadata: EncryptedMetadata::random(),
        to_node_metadata: EncryptedMetadata::random(),
        from_node_change_counter: 0,
        to_node_change_counter: 0,
    };

    let response = user
        .post(routes::node::move_to(folder1.id))
        .json(&payload)
        .await;

    assert_eq!(response.status_code(), StatusCode::CONFLICT);
}

#[tokio::test]
pub async fn test_move_to_descendants() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let parent_folder = user.generate_random_folder().await;
    let child_folder = user.generate_folder_in(parent_folder.id).await;

    let payload = PostMoveNodeRequest {
        to_node_id: child_folder.id,
        from_node_metadata: EncryptedMetadata::random(),
        to_node_metadata: EncryptedMetadata::random(),
        from_node_change_counter: 0,
        to_node_change_counter: 0,
    };

    let response = user
        .post(routes::node::move_to(parent_folder.id))
        .json(&payload)
        .await;

    assert_eq!(response.status_code(), StatusCode::CONFLICT);
}

#[tokio::test]
pub async fn test_move_invalid_node() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let target_folder = user.generate_random_folder().await;

    let payload = PostMoveNodeRequest {
        to_node_id: target_folder.id,
        from_node_metadata: EncryptedMetadata::random(),
        to_node_metadata: EncryptedMetadata::random(),
        from_node_change_counter: 0,
        to_node_change_counter: 0,
    };

    let response = user
        .post(routes::node::move_to(UUID::random()))
        .json(&payload)
        .await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
pub async fn test_move_from_invalid_node() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let file = user.generate_random_file().await;

    let payload = PostMoveNodeRequest {
        to_node_id: file.id,
        from_node_metadata: EncryptedMetadata::random(),
        to_node_metadata: EncryptedMetadata::random(),
        from_node_change_counter: 0,
        to_node_change_counter: 0,
    };

    let response = user
        .post(routes::node::move_to(UUID::random()))
        .json(&payload)
        .await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
pub async fn test_move_to_invalid_node() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let target_folder1 = user.generate_random_folder().await;
    let target_file1 = user.generate_file_in(target_folder1.id).await;

    let payload = PostMoveNodeRequest {
        to_node_id: UUID::random(),
        from_node_metadata: EncryptedMetadata::random(),
        to_node_metadata: EncryptedMetadata::random(),
        from_node_change_counter: 0,
        to_node_change_counter: 0,
    };

    let response = user
        .post(routes::node::move_to(target_file1.id))
        .json(&payload)
        .await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
pub async fn test_move_node_with_invalid_metadata_change_counter() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let target_folder1 = user.generate_random_folder().await;
    let target_file1 = user.generate_file_in(target_folder1.id).await;

    let payload = PostMoveNodeRequest {
        to_node_id: target_folder1.id,
        from_node_metadata: EncryptedMetadata::random(),
        to_node_metadata: EncryptedMetadata::random(),
        from_node_change_counter: 999,
        to_node_change_counter: 999,
    };

    let response = user
        .post(routes::node::move_to(target_file1.id))
        .json(&payload)
        .await;

    assert_eq!(response.status_code(), StatusCode::CONFLICT);
}

#[tokio::test]
pub async fn test_move_root_and_trash_node() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let target_folder1 = user.generate_random_folder().await;

    let payload = PostMoveNodeRequest {
        to_node_id: target_folder1.id,
        from_node_metadata: EncryptedMetadata::random(),
        to_node_metadata: EncryptedMetadata::random(),
        from_node_change_counter: 0,
        to_node_change_counter: 0,
    };

    let response = user
        .post(routes::node::move_to(user.get_root()))
        .json(&payload)
        .await;

    response.assert_status_internal_server_error();

    let payload = PostMoveNodeRequest {
        to_node_id: target_folder1.id,
        from_node_metadata: EncryptedMetadata::random(),
        to_node_metadata: EncryptedMetadata::random(),
        from_node_change_counter: 0,
        to_node_change_counter: 0,
    };

    let response = user
        .post(routes::node::move_to(user.get_trash()))
        .json(&payload)
        .await;

    response.assert_status_internal_server_error();
}

#[tokio::test]
pub async fn test_move_node_to_trash() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let folder = user.generate_random_folder().await;
    let root_node = user.fetch_node_from_db(user.get_root()).unwrap();
    let trash_node = user.fetch_node_from_db(user.get_trash()).unwrap();

    let payload = PostMoveNodeToTrashRequest {
        to_node_id: trash_node.id,
        from_node_metadata: EncryptedMetadata::random(),
        to_node_metadata: EncryptedMetadata::random(),
        from_node_change_counter: root_node.metadata_change_counter,
        to_node_change_counter: trash_node.metadata_change_counter,
    };

    let response = user
        .post(routes::node::move_to_trash(folder.id))
        .json(&payload)
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let PostMoveNodeToTrashResponse::Ok = response.json() else {
        panic!("Expected Ok");
    };

    let get_resp = user.get(routes::node::by_id(folder.id)).await;
    assert_eq!(get_resp.status_code(), StatusCode::OK);
    let GetNodeResponse::Ok(moved_node) = get_resp.json() else {
        panic!("Expected Ok")
    };
    assert_eq!(moved_node.parent_id.unwrap(), trash_node.id);
}

#[tokio::test]
pub async fn test_move_node_out_of_trash() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let folder = user.generate_random_folder().await;
    let root_node = user.fetch_node_from_db(user.get_root()).unwrap();
    let trash_node = user.fetch_node_from_db(user.get_trash()).unwrap();

    let payload_to_trash = PostMoveNodeToTrashRequest {
        to_node_id: trash_node.id,
        from_node_metadata: EncryptedMetadata::random(),
        to_node_metadata: EncryptedMetadata::random(),
        from_node_change_counter: root_node.metadata_change_counter,
        to_node_change_counter: trash_node.metadata_change_counter,
    };

    user.post(routes::node::move_to_trash(folder.id))
        .json(&payload_to_trash)
        .await
        .assert_status_ok();

    let root_node = user.fetch_node_from_db(user.get_root()).unwrap();
    let trash_node = user.fetch_node_from_db(user.get_trash()).unwrap();

    let payload_out_of_trash = PostMoveNodeOutOfTrashRequest {
        to_node_id: root_node.id,
        from_node_metadata: EncryptedMetadata::random(),
        to_node_metadata: EncryptedMetadata::random(),
        from_node_change_counter: trash_node.metadata_change_counter,
        to_node_change_counter: root_node.metadata_change_counter,
    };

    let response = user
        .post(routes::node::move_out_of_trash(folder.id))
        .json(&payload_out_of_trash)
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let PostMoveNodeOutOfTrashResponse::Ok = response.json() else {
        panic!("Expected Ok");
    };

    let get_resp = user.get(routes::node::by_id(folder.id)).await;
    assert_eq!(get_resp.status_code(), StatusCode::OK);
    let GetNodeResponse::Ok(moved_node) = get_resp.json() else {
        panic!("Expected Ok")
    };
    assert_eq!(moved_node.parent_id.unwrap(), root_node.id);
}

// get children of a node

#[tokio::test]
pub async fn test_get_children() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let folder = user.generate_random_folder().await;
    let child1 = user.generate_file_in(folder.id).await;
    let child2 = user.generate_folder_in(folder.id).await;
    let child3 = user.generate_folder_in(folder.id).await;

    let mut ids = vec![child1.id, child2.id, child3.id];
    ids.sort();

    let response = user.get(routes::node::children(folder.id)).await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let returned_nodes = match response.json::<GetNodeChildrenResponse>() {
        GetNodeChildrenResponse::Ok(encrypted_nodes) => encrypted_nodes,
        _ => panic!("Wrong HTTP status code"),
    };

    let mut sent_ids = returned_nodes.iter().map(|f| f.id).collect::<Vec<UUID>>();
    sent_ids.sort();

    assert_eq!(ids, sent_ids)
}

#[tokio::test]
pub async fn test_get_children_invalid() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let response = user.get(routes::node::children(UUID::random())).await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
pub async fn test_get_children_of_file() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let file = user.generate_random_file().await;
    let response = user.get(routes::node::children(file.id)).await;

    assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
}

// get accessible path

#[tokio::test]
pub async fn test_accessible_path() {
    let ctx = TestContext::new(1).await;
    let user1 = ctx.get_user(0);

    let mut created_nodes: Vec<UUID> = Vec::new();

    let mut parent = user1.get_root();
    created_nodes.push(parent);
    for _ in 0..10 {
        let child = user1.generate_folder_in(parent).await;
        created_nodes.push(child.id);
        parent = child.id;
    }

    let request = user1.get(routes::node::accessible_path(parent)).await;

    request.assert_status_ok();
    let nodes = match request.json::<GetAccessiblePathResponse>() {
        GetAccessiblePathResponse::Ok(encrypted_nodes) => encrypted_nodes,
        GetAccessiblePathResponse::NotFound => panic!("Invalid node!"),
    };

    let nodes = nodes.iter().map(|f| f.id).collect::<Vec<UUID>>();
    assert_eq!(nodes, created_nodes);
}

// other

#[tokio::test]
pub async fn test_unauthorized_access() {
    let ctx = TestContext::new(2).await;
    let user1 = ctx.get_user(0);
    let user2 = ctx.get_user(1);

    let file = user1.generate_random_file().await;

    let response = user2.get(routes::node::by_id(file.id)).await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
}
