use crate::test::utils::TestContext;

use crabdrive_common::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::iv::IV;
use crabdrive_common::payloads::node::{
    request::{file::*, folder::*, node::*},
    response::{file::*, folder::*, node::*},
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

    let id = UUID::random();

    let create_folder_body = PostCreateFolderRequest {
        parent_metadata_version: 0,
        parent_metadata: EncryptedMetadata::random(),
        node_metadata: EncryptedMetadata::random(),
        node_id: id,
    };

    let request = user1
        .post(routes::node::folder::create(user1.get_root()))
        .json(&create_folder_body)
        .await;

    assert_eq!(request.status_code(), StatusCode::CREATED);
    assert!(ctx.node.get_node(id).expect("Failed to get node").is_some())
}

#[tokio::test]
pub async fn test_create_file() {
    let ctx = TestContext::new(1).await;

    let user1 = ctx.get_user(0);

    let id = UUID::random();

    let create_file_body = PostCreateFileRequest {
        parent_metadata_version: 0,
        parent_metadata: EncryptedMetadata::random(),
        node_metadata: EncryptedMetadata::random(),
        node_id: id,
        file_iv: IV::random(),
        chunk_count: 0,
    };

    let request = user1
        .post(routes::node::file::create(user1.get_root()))
        .json(&create_file_body)
        .await;

    assert_eq!(request.status_code(), StatusCode::CREATED);
}

#[tokio::test]
pub async fn test_create_file_in_random_parent() {
    let ctx = TestContext::new(1).await;

    let user1 = ctx.get_user(0);

    let create_file_body = PostCreateFileRequest {
        parent_metadata_version: 0,
        parent_metadata: EncryptedMetadata::random(),
        node_metadata: EncryptedMetadata::random(),
        node_id: UUID::random(),
        file_iv: IV::random(),
        chunk_count: 1,
    };

    let request = user1
        .post(routes::node::file::create(UUID::random()))
        .json(&create_file_body)
        .await;

    assert_eq!(request.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
pub async fn test_create_file_in_file() {
    let ctx = TestContext::new(1).await;

    let user1 = ctx.get_user(0);
    let file = user1.generate_random_file().await;

    let create_file_body = PostCreateFileRequest {
        parent_metadata_version: 0,
        parent_metadata: EncryptedMetadata::random(),
        node_metadata: EncryptedMetadata::random(),
        node_id: UUID::random(),
        file_iv: IV::random(),
        chunk_count: 1,
    };

    let request = user1
        .post(routes::node::file::create(file.id))
        .json(&create_file_body)
        .await;

    assert_eq!(request.status_code(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
pub async fn test_create_file_with_negative_chunk_count() {
    let ctx = TestContext::new(1).await;

    let user1 = ctx.get_user(0);
    let file = user1.generate_random_file().await;

    let create_file_body = PostCreateFileRequest {
        parent_metadata_version: 0,
        parent_metadata: EncryptedMetadata::random(),
        node_metadata: EncryptedMetadata::random(),
        node_id: UUID::random(),
        file_iv: IV::random(),
        chunk_count: -1,
    };

    let request = user1
        .post(routes::node::file::create(file.id))
        .json(&create_file_body)
        .await;

    assert_eq!(request.status_code(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
pub async fn test_create_file_and_upload_single_chunk() {
    let ctx = TestContext::new(1).await;

    let user1 = ctx.get_user(0);

    let id = UUID::random();
    let metadata = EncryptedMetadata::random();

    let create_file_body = PostCreateFileRequest {
        parent_metadata_version: 0,
        parent_metadata: EncryptedMetadata::random(),
        node_metadata: metadata.clone(),
        node_id: id,
        file_iv: IV::random(),
        chunk_count: 1,
    };

    let request = user1
        .post(routes::node::file::create(user1.get_root()))
        .json(&create_file_body)
        .await;

    assert_eq!(request.status_code(), StatusCode::CREATED);

    assert!(ctx.node.get_node(id).expect("Failed to get node").is_some());

    let created_node = match request.json::<PostCreateFileResponse>() {
        PostCreateFileResponse::Created(encrypted_node) => encrypted_node,
        PostCreateFileResponse::NotFound => panic!("Wrong status code!"),
        PostCreateFileResponse::BadRequest => panic!("Wrong status code!"),
        PostCreateFileResponse::Conflict => panic!("Wrong status code!"),
    };

    assert!(created_node.current_revision.is_some());
    assert!(created_node.parent_id.is_some());
    assert!(created_node.deleted_on.is_none());
    assert_eq!(created_node.node_type, NodeType::File);
    assert_eq!(created_node.encrypted_metadata, metadata);

    let current_revision = created_node.current_revision.unwrap().id;

    let bytes = TestContext::random_bytes(4096);
    let request = user1
        .post(routes::node::chunks(id, current_revision, 1))
        .bytes(bytes)
        .await;

    assert_eq!(request.status_code(), StatusCode::CREATED);

    let request = user1
        .post(routes::node::file::commit(id, current_revision))
        .json(&create_file_body)
        .await;

    assert_eq!(request.status_code(), StatusCode::OK);
}

#[tokio::test]
pub async fn test_create_file_and_upload_chunks() {
    let ctx = TestContext::new(1).await;

    let user1 = ctx.get_user(0);

    let id = UUID::random();
    let chunks = TestContext::random_range(5..15);
    let metadata = EncryptedMetadata::random();

    let create_file_body = PostCreateFileRequest {
        parent_metadata_version: 0,
        parent_metadata: EncryptedMetadata::random(),
        node_metadata: metadata.clone(),
        node_id: id,
        file_iv: IV::random(),
        chunk_count: chunks as i64,
    };

    let request = user1
        .post(routes::node::file::create(user1.get_root()))
        .json(&create_file_body)
        .await;

    assert_eq!(request.status_code(), StatusCode::CREATED);

    assert!(ctx.node.get_node(id).expect("Failed to get node").is_some());

    let created_node = match request.json::<PostCreateFileResponse>() {
        PostCreateFileResponse::Created(encrypted_node) => encrypted_node,
        PostCreateFileResponse::NotFound => panic!("Wrong status code!"),
        PostCreateFileResponse::BadRequest => panic!("Wrong status code!"),
        PostCreateFileResponse::Conflict => panic!("Wrong status code!"),
    };

    assert!(created_node.current_revision.is_some());
    assert!(created_node.parent_id.is_some());
    assert!(created_node.deleted_on.is_none());
    assert_eq!(created_node.node_type, NodeType::File);
    assert_eq!(created_node.encrypted_metadata, metadata);

    let current_revision = created_node.current_revision.unwrap().id;

    let mut collected_bytes = Vec::new();

    for i in 1..chunks {
        let bytes = TestContext::random_bytes(4096);
        collected_bytes.push(bytes.clone());
        let request = user1
            .post(routes::node::chunks(id, current_revision, i as i64))
            .bytes(bytes)
            .await;

        assert_eq!(request.status_code(), StatusCode::CREATED);
    }

    let request = user1
        .post(routes::node::file::commit(id, current_revision))
        .json(&create_file_body)
        .await;

    assert_eq!(request.status_code(), StatusCode::OK);

    for i in 1..chunks {
        let bytes_from_vfs = ctx
            .state
            .vfs
            .read()
            .await
            .read_chunk(&current_revision, i as i64)
            .await
            .expect("Failed to read from VFS!");

        assert_eq!(bytes_from_vfs.index, i as i64);
        assert_eq!(bytes_from_vfs.data, collected_bytes[i - 1]);
    }
}

#[tokio::test]
pub async fn test_create_file_and_not_upload_all_chunks() {
    let ctx = TestContext::new(1).await;

    let user1 = ctx.get_user(0);

    let id = UUID::random();
    let chunks = TestContext::random_range(20..40);
    let missing = 25..30;
    let metadata = EncryptedMetadata::random();

    let create_file_body = PostCreateFileRequest {
        parent_metadata_version: 0,
        parent_metadata: EncryptedMetadata::random(),
        node_metadata: metadata.clone(),
        node_id: id,
        file_iv: IV::random(),
        chunk_count: chunks as i64,
    };

    let request = user1
        .post(routes::node::file::create(user1.get_root()))
        .json(&create_file_body)
        .await;

    assert_eq!(request.status_code(), StatusCode::CREATED);

    assert!(ctx.node.get_node(id).expect("Failed to get node").is_some());

    let created_node = match request.json::<PostCreateFileResponse>() {
        PostCreateFileResponse::Created(encrypted_node) => encrypted_node,
        PostCreateFileResponse::NotFound => panic!("Wrong status code!"),
        PostCreateFileResponse::BadRequest => panic!("Wrong status code!"),
        PostCreateFileResponse::Conflict => panic!("Wrong status code!"),
    };

    assert!(created_node.current_revision.is_some());
    assert!(created_node.parent_id.is_some());
    assert!(created_node.deleted_on.is_none());
    assert_eq!(created_node.node_type, NodeType::File);
    assert_eq!(created_node.encrypted_metadata, metadata);

    let current_revision = created_node.current_revision.unwrap().id;

    for i in 1..chunks {
        if missing.contains(&i) {
            // Skip missing chunks
            continue;
        }

        let bytes = TestContext::random_bytes(4096);

        let request = user1
            .post(routes::node::chunks(id, current_revision, i as i64))
            .bytes(bytes)
            .await;

        dbg!(&request);

        assert_eq!(request.status_code(), StatusCode::CREATED);
    }

    let request = user1
        .post(routes::node::file::commit(id, current_revision))
        .json(&create_file_body)
        .await;

    assert_eq!(request.status_code(), StatusCode::BAD_REQUEST);

    let commit_err = match request.json::<PostCommitFileResponse>() {
        PostCommitFileResponse::Ok(_) => panic!("Wrong status code!"),
        PostCommitFileResponse::BadRequest(commit_file_error) => commit_file_error,
        PostCommitFileResponse::NotFound => panic!("Wrong status code!"),
    };

    let CommitFileError::MissingChunks(mut server_missing_chunks) = commit_err else {
        panic!("Invalid response!");
    };
    let mut test_missing_chunks: Vec<i64> = missing.map(|x| x as i64).collect();

    server_missing_chunks.sort();
    test_missing_chunks.sort();

    assert_eq!(server_missing_chunks, test_missing_chunks);
}

#[tokio::test]
pub async fn test_create_file_and_upload_wrong_chunk_indexes() {
    let ctx = TestContext::new(1).await;

    let user1 = ctx.get_user(0);

    let id = UUID::random();
    let chunks = TestContext::random_range(2..5);
    let metadata = EncryptedMetadata::random();

    let create_file_body = PostCreateFileRequest {
        parent_metadata_version: 0,
        parent_metadata: EncryptedMetadata::random(),
        node_metadata: metadata.clone(),
        node_id: id,
        file_iv: IV::random(),
        chunk_count: chunks as i64,
    };

    let request = user1
        .post(routes::node::file::create(user1.get_root()))
        .json(&create_file_body)
        .await;

    assert_eq!(request.status_code(), StatusCode::CREATED);

    assert!(ctx.node.get_node(id).expect("Failed to get node").is_some());

    let created_node = match request.json::<PostCreateFileResponse>() {
        PostCreateFileResponse::Created(encrypted_node) => encrypted_node,
        PostCreateFileResponse::NotFound => panic!("Wrong status code!"),
        PostCreateFileResponse::BadRequest => panic!("Wrong status code!"),
        PostCreateFileResponse::Conflict => panic!("Wrong status code!"),
    };

    assert!(created_node.current_revision.is_some());
    assert!(created_node.parent_id.is_some());
    assert!(created_node.deleted_on.is_none());
    assert_eq!(created_node.node_type, NodeType::File);
    assert_eq!(created_node.encrypted_metadata, metadata);

    let current_revision = created_node.current_revision.unwrap().id;

    for _ in 1..chunks {
        let bytes = TestContext::random_bytes(128);

        let request = user1
            .post(routes::node::chunks(
                id,
                current_revision,
                TestContext::random_range(10..20) as i64,
            ))
            .bytes(bytes)
            .await;

        assert_eq!(request.status_code(), StatusCode::BAD_REQUEST);
    }
}

#[tokio::test]
pub async fn test_upload_chunks_to_invalid_revision() {
    let ctx = TestContext::new(1).await;

    let user1 = ctx.get_user(0);

    let id = UUID::random();
    let metadata = EncryptedMetadata::random();

    let create_file_body = PostCreateFileRequest {
        parent_metadata_version: 0,
        parent_metadata: EncryptedMetadata::random(),
        node_metadata: metadata.clone(),
        node_id: id,
        file_iv: IV::random(),
        chunk_count: 1,
    };

    let request = user1
        .post(routes::node::file::create(user1.get_root()))
        .json(&create_file_body)
        .await;

    assert_eq!(request.status_code(), StatusCode::CREATED);

    assert!(ctx.node.get_node(id).expect("Failed to get node").is_some());

    let created_node = match request.json::<PostCreateFileResponse>() {
        PostCreateFileResponse::Created(encrypted_node) => encrypted_node,
        PostCreateFileResponse::NotFound => panic!("Wrong status code!"),
        PostCreateFileResponse::BadRequest => panic!("Wrong status code!"),
        PostCreateFileResponse::Conflict => panic!("Wrong status code!"),
    };

    assert!(created_node.current_revision.is_some());
    assert!(created_node.parent_id.is_some());
    assert!(created_node.deleted_on.is_none());
    assert_eq!(created_node.node_type, NodeType::File);
    assert_eq!(created_node.encrypted_metadata, metadata);

    let current_revision = UUID::random();

    let bytes = TestContext::random_bytes(4096);
    let request = user1
        .post(routes::node::chunks(id, current_revision, 0))
        .bytes(bytes)
        .await;

    assert_eq!(request.status_code(), StatusCode::NOT_FOUND);

    let request = user1
        .post(routes::node::file::commit(id, current_revision))
        .json(&create_file_body)
        .await;

    assert_eq!(request.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_double_commit() {
    let ctx = TestContext::new(1).await;

    let user1 = ctx.get_user(0);

    let id = UUID::random();
    let metadata = EncryptedMetadata::random();

    let create_file_body = PostCreateFileRequest {
        parent_metadata_version: 0,
        parent_metadata: EncryptedMetadata::random(),
        node_metadata: metadata.clone(),
        node_id: id,
        file_iv: IV::random(),
        chunk_count: 1,
    };

    let request = user1
        .post(routes::node::file::create(user1.get_root()))
        .json(&create_file_body)
        .await;

    assert_eq!(request.status_code(), StatusCode::CREATED);

    assert!(ctx.node.get_node(id).expect("Failed to get node").is_some());

    let created_node = match request.json::<PostCreateFileResponse>() {
        PostCreateFileResponse::Created(encrypted_node) => encrypted_node,
        PostCreateFileResponse::NotFound => panic!("Wrong status code!"),
        PostCreateFileResponse::BadRequest => panic!("Wrong status code!"),
        PostCreateFileResponse::Conflict => panic!("Wrong status code!"),
    };

    assert!(created_node.current_revision.is_some());
    assert!(created_node.parent_id.is_some());
    assert!(created_node.deleted_on.is_none());
    assert_eq!(created_node.node_type, NodeType::File);
    assert_eq!(created_node.encrypted_metadata, metadata);

    let current_revision = created_node.current_revision.unwrap().id;

    let bytes = TestContext::random_bytes(4096);
    let request = user1
        .post(routes::node::chunks(id, current_revision, 1))
        .bytes(bytes)
        .await;

    assert_eq!(request.status_code(), StatusCode::CREATED);

    let request = user1
        .post(routes::node::file::commit(id, current_revision))
        .json(&create_file_body)
        .await;

    assert_eq!(request.status_code(), StatusCode::OK);

    let request = user1
        .post(routes::node::file::commit(id, current_revision))
        .json(&create_file_body)
        .await;

    assert_eq!(request.status_code(), StatusCode::BAD_REQUEST);

    let commit_err = match request.json::<PostCommitFileResponse>() {
        PostCommitFileResponse::Ok(_) => panic!("Wrong status code!"),
        PostCommitFileResponse::BadRequest(commit_file_error) => commit_file_error,
        PostCommitFileResponse::NotFound => panic!("Wrong status code!"),
    };

    assert_eq!(commit_err, CommitFileError::AlreadyCommitted);
}

#[tokio::test]
pub async fn test_download_file() {
    let ctx = TestContext::new(1).await;

    let user1 = ctx.get_user(0);

    let chunks = TestContext::random_range(2..10);

    let file1 = user1.generate_file_with_chunks(chunks as u32).await;
    let revision1 = file1.active_revision.expect("No revision with file!");

    for i in 1..chunks {
        let chunk = &revision1.chunks[i];

        let request = user1
            .get(routes::node::chunks(file1.id, revision1.id, i as i64))
            .await;

        assert_eq!(request.status_code(), StatusCode::OK);

        let body = request.as_bytes();
        TestContext::validate_checksum(&chunk.checksum, body);
    }
}

#[tokio::test]
pub async fn test_parent_metadata_mismatch() {
    let ctx = TestContext::new(1).await;

    let user1 = ctx.get_user(0);

    let id = UUID::random();

    let create_file_body = PostCreateFileRequest {
        parent_metadata_version: 0,
        parent_metadata: EncryptedMetadata::random(),
        node_metadata: EncryptedMetadata::random(),
        node_id: id,
        file_iv: IV::random(),
        chunk_count: 0,
    };

    let request = user1
        .post(routes::node::file::create(user1.get_root()))
        .json(&create_file_body)
        .await;

    assert_eq!(request.status_code(), StatusCode::CREATED);

    let create_file_body_2 = PostCreateFileRequest {
        parent_metadata_version: 0,
        parent_metadata: EncryptedMetadata::random(),
        node_metadata: EncryptedMetadata::random(),
        node_id: id,
        file_iv: IV::random(),
        chunk_count: 0,
    };

    let request = user1
        .post(routes::node::file::create(user1.get_root()))
        .json(&create_file_body_2)
        .await;

    assert_eq!(request.status_code(), StatusCode::CONFLICT);
}

#[tokio::test]
pub async fn test_move_file() {
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

    assert_eq!(response.status_code(), StatusCode::OK);
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

    let response = user
        .post(routes::node::move_to(file1.id))
        .json(&payload)
        .await;

    dbg!(response);

    // Depending on DB/repository constraints, this might be OK from the handler's perspective
    // but should logically fail. We'll assert OK or an error depending on your domain logic.
    // For now, testing what the handler allows.
    assert!(false);
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

    assert_eq!(response.status_code(), StatusCode::OK);
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
pub async fn test_move_root_node() {
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

    assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
pub async fn test_move_trash_node() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    todo!()
}

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
    let ctx = TestContext::new(2).await;

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

#[tokio::test]
pub async fn test_get_children() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let folder = user.generate_random_folder().await;
    let _child1 = user.generate_file_in(folder.id).await;
    let _child2 = user.generate_folder_in(folder.id).await;

    let response = user.get(routes::node::children(folder.id)).await;

    assert_eq!(response.status_code(), StatusCode::OK);
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

    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
pub async fn test_unauthorized_access() {
    let ctx = TestContext::new(2).await;
    let user1 = ctx.get_user(0);
    let user2 = ctx.get_user(1);

    let file = user1.generate_random_file().await;

    let response = user2.get(routes::node::by_id(file.id)).await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
}
