use crate::auth::secrets::Keys;
use crate::db::connection::create_pool;
use crate::http::middleware::logging_middleware;
use crate::http::{AppConfig, AppState, routes};
use crate::storage::node::persistence::model::node_entity::NodeEntity;
use crate::storage::node::persistence::node_repository::NodeState;
use crate::storage::revision::persistence::revision_repository::RevisionService;
use crate::storage::vfs::backend::Sfs;
use crate::user::persistence::model::user_entity::UserEntity;
use crate::user::persistence::user_repository::UserState;
use axum::http::StatusCode;
use axum::{Router, middleware};
use axum_test::TestServer;
use bytes::Bytes;
use chrono::Local;
use crabdrive_common::da;
use crabdrive_common::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::encryption_key::EncryptionKey;
use crabdrive_common::iv::IV;
use crabdrive_common::payloads::auth::request::login::PostLoginRequest;
use crabdrive_common::payloads::auth::request::register::PostRegisterRequest;
use crabdrive_common::payloads::auth::response::info::GetSelfInfoResponse;
use crabdrive_common::payloads::auth::response::login::{PostLoginResponse, UserKeys};
use crabdrive_common::payloads::auth::response::register::{
    PostRegisterResponse, RegisterConflictReason,
};
use crabdrive_common::payloads::node::request::file::PostCreateFileRequest;
use crabdrive_common::payloads::node::request::folder::PostCreateFolderRequest;
use crabdrive_common::payloads::node::response::file::CommitFileError::AlreadyCommitted;
use crabdrive_common::payloads::node::response::file::{
    PostCommitFileResponse, PostCreateFileResponse,
};
use crabdrive_common::payloads::node::response::folder::PostCreateFolderResponse;
use crabdrive_common::payloads::node::response::node::{
    GetNodeResponse, GetPathBetweenNodesResponse,
};
use crabdrive_common::routes::{
    CHUNK_ROUTE, COMMIT_FILE_ROUTE, CREATE_FILE_ROUTE, CREATE_FOLDER_ROUTE, LOGIN_ROUTE,
    NODE_ROUTE_NODEID, PATH_BETWEEN_NODES_ROUTE, REGISTER_ROUTE, USER_INFO_ROUTE,
};
use crabdrive_common::storage::{EncryptedNode, NodeId, NodeType};
use crabdrive_common::uuid::UUID;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use formatx::formatx;
use pretty_assertions::assert_eq;
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::catch_panic::CatchPanicLayer;

const API_BASE_PATH: &str = "http://localhost:2722";

pub fn random_metadata() -> EncryptedMetadata {
    let mut data = vec![0; 6403];
    rand::rng().fill_bytes(&mut data);
    let mut iv_bytes = [0; 12];
    rand::rng().fill_bytes(&mut iv_bytes);
    let iv = IV::new(iv_bytes);
    EncryptedMetadata::new(data, iv)
}

#[tokio::test]
pub async fn test_register() {
    let server = get_server();

    let username = "test_user";
    let password = "test_password";

    let register = PostRegisterRequest {
        username: username.to_string(),
        password: password.to_string(),
        keys: UserKeys::nil(),
    };

    let register_url = API_BASE_PATH.to_owned() + REGISTER_ROUTE;

    let register_request = server.post(&register_url).json(&register).await;
    let register_response: PostRegisterResponse = register_request.json();
    assert_eq!(register_response, PostRegisterResponse::Created);

    let register_request2 = server.post(&register_url).json(&register).await;
    let register_response2: PostRegisterResponse = register_request2.json();
    assert_eq!(
        register_response2,
        PostRegisterResponse::Conflict(RegisterConflictReason::UsernameTaken)
    );

    let login = PostLoginRequest {
        username: username.to_string(),
        password: password.to_string(),
    };

    let login_url = API_BASE_PATH.to_owned() + LOGIN_ROUTE;

    let login_request = server.post(&login_url).json(&login).await;
    let login_response: PostLoginResponse = login_request.json();

    if let PostLoginResponse::Ok(response) = login_response {
        let jwt = response.bearer_token;

        let user_info_url = API_BASE_PATH.to_owned() + USER_INFO_ROUTE;
        let user_info_request = server
            .get(&user_info_url)
            .add_header("Authorization", format!("Bearer {}", jwt))
            .await;

        let user_info_response: GetSelfInfoResponse = user_info_request.json();

        let GetSelfInfoResponse::Ok(info) = user_info_response;
        assert_eq!(info.username, username.to_string());
    } else {
        unreachable!("login_response should be OK");
    }
}

#[tokio::test]
pub async fn test_folder() {
    let server = get_server();

    let parent_metadata = random_metadata();
    let node_metadata = random_metadata();
    let node_id = UUID::random();

    let create_node_request = PostCreateFolderRequest {
        parent_metadata_version: 0,
        parent_metadata,
        node_metadata: node_metadata.clone(),
        node_id,
    };

    let url = API_BASE_PATH.to_owned() + &formatx!(CREATE_FOLDER_ROUTE, UUID::nil()).unwrap();

    let test_request = server.post(&url).json(&create_node_request).await;

    let create_folder_response: PostCreateFolderResponse = test_request.json();

    let expected_response = PostCreateFolderResponse::Created(EncryptedNode {
        id: node_id,
        change_count: 0,
        parent_id: Some(UUID::nil()),
        owner_id: UUID::nil(),
        deleted_on: None,
        node_type: NodeType::Folder,
        current_revision: None,
        encrypted_metadata: node_metadata,
    });

    assert_eq!(expected_response, create_folder_response);
}

#[tokio::test]
pub async fn test_path_between_nodes() {
    let server = get_server();

    let create_node_request1 = PostCreateFolderRequest {
        parent_metadata_version: 0,
        parent_metadata: random_metadata(),
        node_metadata: random_metadata(),
        node_id: NodeId::random(),
    };

    let create_node_request2 = PostCreateFolderRequest {
        parent_metadata_version: 0,
        parent_metadata: random_metadata(),
        node_metadata: random_metadata(),
        node_id: NodeId::random(),
    };

    let create_node_request3 = PostCreateFolderRequest {
        parent_metadata_version: 1,
        parent_metadata: random_metadata(),
        node_metadata: random_metadata(),
        node_id: NodeId::random(),
    };

    let create_folder_in_root_url =
        API_BASE_PATH.to_owned() + &formatx!(CREATE_FOLDER_ROUTE, UUID::nil()).unwrap();
    let create_node_request1_response = server
        .post(&create_folder_in_root_url)
        .json(&create_node_request1)
        .await;

    assert_eq!(
        create_node_request1_response.status_code(),
        StatusCode::CREATED
    );

    let create_folder_url = API_BASE_PATH.to_owned()
        + &formatx!(CREATE_FOLDER_ROUTE, create_node_request1.node_id).unwrap();

    let create_node_request2_response = server
        .post(&create_folder_url)
        .json(&create_node_request2)
        .await;
    assert_eq!(
        create_node_request2_response.status_code(),
        StatusCode::CREATED
    );

    let create_node_request3_response = server
        .post(&create_folder_in_root_url)
        .json(&create_node_request3)
        .await;
    assert_eq!(
        create_node_request3_response.status_code(),
        StatusCode::CREATED
    );

    let path_between_nodes_url1 = API_BASE_PATH.to_owned()
        + &formatx!(
            "{}?from_id={}&to_id={}",
            PATH_BETWEEN_NODES_ROUTE,
            UUID::nil(),
            create_node_request2.node_id
        )
        .unwrap();
    let path_between_nodes_response1 = server.get(&path_between_nodes_url1).await;

    let path_between_nodes_url2 = API_BASE_PATH.to_owned()
        + &formatx!(
            "{}?from_id={}&to_id={}",
            PATH_BETWEEN_NODES_ROUTE,
            create_node_request3.node_id,
            create_node_request2.node_id
        )
        .unwrap();
    let path_between_nodes_response2 = server.get(&path_between_nodes_url2).await;

    match path_between_nodes_response1.json::<GetPathBetweenNodesResponse>() {
        GetPathBetweenNodesResponse::Ok(path) => {
            assert_eq!(path[0].id, UUID::nil());
            assert_eq!(path[1].id, create_node_request1.node_id);
            assert_eq!(path[2].id, create_node_request2.node_id);
        }
        _ => {
            panic!("unexpected response");
        }
    }

    assert_eq!(
        path_between_nodes_response2.json::<GetPathBetweenNodesResponse>(),
        GetPathBetweenNodesResponse::NoContent
    );
}

#[tokio::test]
pub async fn test_file() {
    let server = get_server();

    let parent_metadata = random_metadata();
    let node_metadata = random_metadata();
    let node_id = UUID::random();

    let create_node_request = PostCreateFileRequest {
        parent_metadata_version: 0,
        parent_metadata,
        node_metadata: node_metadata.clone(),
        file_iv: IV::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]),
        chunk_count: 2,
        node_id,
    };

    let create_file_url =
        API_BASE_PATH.to_owned() + &formatx!(CREATE_FILE_ROUTE, UUID::nil()).unwrap();

    let test_request = server
        .post(&create_file_url)
        .json(&create_node_request)
        .await;

    let create_file_response: PostCreateFileResponse = test_request.json();

    let revision;
    match create_file_response {
        PostCreateFileResponse::Created(node) => {
            assert_eq!(node.encrypted_metadata, node_metadata);
            assert_eq!(node.id, node_id);
            assert_eq!(node.node_type, NodeType::File);
            assert!(node.current_revision.is_some());
            revision = node.current_revision.unwrap();
        }
        _ => {
            panic!("failed to create file")
        }
    }

    let mut chunk1 = vec![0; da!(16 MB).as_bytes() as usize];
    let mut chunk2 = vec![0; da!(16 MB).as_bytes() as usize];

    let mut rng = SmallRng::from_rng(&mut rand::rng());

    rng.fill_bytes(&mut chunk1);
    rng.fill_bytes(&mut chunk2);

    let chunk1 = Bytes::from(chunk1);
    let chunk2 = Bytes::from(chunk2);

    let chunk_url1 = API_BASE_PATH.to_owned()
        + &formatx!(CHUNK_ROUTE, create_node_request.node_id, revision.id, 1).unwrap();
    let chunk_url2 = API_BASE_PATH.to_owned()
        + &formatx!(CHUNK_ROUTE, create_node_request.node_id, revision.id, 2).unwrap();

    let chunk1_response = server.post(&chunk_url1).bytes(chunk1.clone()).await;
    assert_eq!(chunk1_response.status_code(), StatusCode::CREATED);

    let chunk2_response = server.post(&chunk_url2).bytes(chunk2.clone()).await;
    assert_eq!(chunk2_response.status_code(), StatusCode::CREATED);

    let commit_file_url = API_BASE_PATH.to_owned()
        + &formatx!(COMMIT_FILE_ROUTE, create_node_request.node_id, revision.id).unwrap();

    let commit_file_response: PostCommitFileResponse = server.post(&commit_file_url).await.json();

    let node;
    match commit_file_response {
        PostCommitFileResponse::Ok(_node) => node = _node,
        _ => {
            panic!("failed to commit file")
        }
    }

    assert_eq!(node.current_revision.as_ref().unwrap().id, revision.id);

    let get_node_url = API_BASE_PATH.to_owned()
        + &formatx!(NODE_ROUTE_NODEID, create_node_request.node_id).unwrap();

    let node_response: GetNodeResponse = server.get(&get_node_url).await.json();

    match node_response {
        GetNodeResponse::Ok(same_node_as_before) => {
            assert_eq!(same_node_as_before, node)
        }
        _ => {
            panic!("failed to get node")
        }
    }

    let get_chunk_response1 = server.get(&chunk_url1).await.into_bytes();
    let get_chunk_response2 = server.get(&chunk_url2).await.into_bytes();

    assert_eq!(get_chunk_response1, chunk1);
    assert_eq!(get_chunk_response2, chunk2);

    // try to commit the file a second time
    let commit_file_response2: PostCommitFileResponse = server.post(&commit_file_url).await.json();
    let expected_commit_file_response2 = PostCommitFileResponse::BadRequest(AlreadyCommitted);
    assert_eq!(commit_file_response2, expected_commit_file_response2);
}

pub fn get_server() -> TestServer {
    let config = AppConfig::load(&PathBuf::from("./crabdrive.toml")).unwrap();

    // https://stackoverflow.com/questions/58649529/how-to-create-multiple-memory-databases-in-sqlite3
    let db_path = format!("file:{}?mode=memory&cache=shared", UUID::random());

    let pool = create_pool(&db_path, config.db.pool_size);

    let vfs = Sfs::new(&config.storage.dir);

    let node_repository = NodeState::new(Arc::new(pool.clone()));
    let revision_repository = RevisionService::new(Arc::new(pool.clone()));
    let user_repository = UserState::new(Arc::new(pool.clone()));

    let keys = Keys::new(&config.auth.jwt_secret);

    let state = AppState::new(
        config.clone(),
        pool,
        vfs,
        node_repository,
        revision_repository,
        user_repository,
        keys,
    );

    prepare_db(&state);

    let app = Router::<AppState>::new()
        .merge(routes::routes())
        .with_state(state.clone())
        .layer(middleware::from_fn(logging_middleware))
        .layer(CatchPanicLayer::custom(crate::http::server::handle_panic));

    TestServer::new(app).unwrap()
}

// copied from server.rs/start
fn prepare_db(state: &AppState) {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./res/migrations/");
    let mut conn = state.db_pool.get().unwrap();
    conn.run_pending_migrations(MIGRATIONS).unwrap();

    // HACK: Create a root node with a zeroed UUID, if it's not already existing.
    // TODO: Remove when adding auth!

    if crate::db::operations::select_user(&state.db_pool, UUID::nil())
        .unwrap()
        .is_none()
    {
        let system_user = UserEntity {
            id: UUID::nil(),
            username: "system".to_string(),
            user_type: crabdrive_common::user::UserType::Admin,
            created_at: Local::now().naive_local(),
            password_hash: "".to_string(),
            storage_limit: da!(500 MB),
            encryption_uninitialized: false,
            master_key: EncryptionKey::nil(),
            private_key: EncryptionKey::nil(),
            public_key: vec![],
            root_key: EncryptionKey::nil(),
            root_node: None,
            trash_key: EncryptionKey::nil(),
            trash_node: None,
        };

        crate::db::operations::insert_user(&state.db_pool, &system_user).unwrap();
    }

    if crate::db::operations::select_node(&state.db_pool, UUID::nil())
        .unwrap()
        .is_none()
    {
        let node = NodeEntity {
            id: UUID::nil(),
            owner_id: UUID::nil(),
            parent_id: None,
            metadata: EncryptedMetadata::nil(),
            deleted_on: None,
            current_revision: None,
            metadata_change_counter: 0,
            node_type: NodeType::Folder,
        };

        crate::db::operations::insert_node(&state.db_pool, &node, &EncryptedMetadata::nil())
            .unwrap();
    }
}
