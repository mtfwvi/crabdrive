use crate::db::connection::create_pool;
use crate::http::middleware::logging_middleware;
use crate::http::{AppConfig, AppState};
use crate::storage::node::persistence::node_repository::NodeState;
use crate::storage::revision::persistence::revision_repository::RevisionService;
use crate::storage::vfs::backend::Sfs;
use crate::user::auth::secrets::Keys;
use crate::user::persistence::user_repository::UserState;
use axum::http::StatusCode;
use axum::{Router, middleware};
use axum_test::{TestRequest, TestServer};
use bytes::Bytes;

use crabdrive_common::da;
use crabdrive_common::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::iv::IV;
use crabdrive_common::payloads::auth::request::login::PostLoginRequest;
use crabdrive_common::payloads::auth::request::register::PostRegisterRequest;
use crabdrive_common::payloads::auth::response::info::GetSelfInfoResponse;
use crabdrive_common::payloads::auth::response::login::PostLoginResponse;
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
    GetAccessiblePathResponse, GetNodeResponse, GetPathBetweenNodesResponse,
};
use crabdrive_common::routes;
use crabdrive_common::storage::{EncryptedNode, NodeId, NodeType};
use crabdrive_common::user::UserKeys;
use crabdrive_common::uuid::UUID;
use std::path::PathBuf;
use std::sync::Arc;

use crate::storage::share::persistence::share_repository::ShareRepositoryImpl;
use crabdrive_common::encryption_key::EncryptionKey;
use crabdrive_common::payloads::node::request::share::{
    PostAcceptShareRequest, PostShareNodeRequest,
};
use crabdrive_common::payloads::node::response::share::{
    GetAcceptShareInfoResponse, GetAcceptedSharedResponse, PostAcceptShareResponse,
    PostShareNodeResponse,
};
use crabdrive_common::routes::auth::{ROUTE_LOGIN, ROUTE_REGISTER};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use pretty_assertions::assert_eq;
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};
use tower_http::catch_panic::CatchPanicLayer;

const API_BASE_PATH: &str = "http://localhost:2722";
const TEST_USERNAME1: &str = "admin";

const TEST_USERNAME2: &str = "admin2";

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
    let server = get_server().await;

    let username = "test_user";
    let password = "test_password";

    let register = PostRegisterRequest {
        username: username.to_string(),
        password: password.to_string(),
        keys: UserKeys::nil(),
    };

    let register_url = API_BASE_PATH.to_owned() + &routes::auth::register();

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

    let login_url = API_BASE_PATH.to_owned() + &routes::auth::login();

    let login_request = server.post(&login_url).json(&login).await;
    let login_response: PostLoginResponse = login_request.json();

    if let PostLoginResponse::Ok(response) = login_response {
        let jwt = response.bearer_token;

        let user_info_url = API_BASE_PATH.to_owned() + &routes::auth::info();
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
    let server = get_server().await;

    let (jwt, root_node_id) = login1(&server).await;

    let get_user_info_request = server
        .get(routes::auth::ROUTE_INFO)
        .add_header("Authorization", format!("Bearer {}", jwt))
        .await;

    let GetSelfInfoResponse::Ok(info) = get_user_info_request.json();
    let self_id = info.user_id;

    let parent_metadata = random_metadata();
    let node_metadata = random_metadata();
    let node_id = UUID::random();

    let create_node_request = PostCreateFolderRequest {
        parent_metadata_version: 0,
        parent_metadata,
        node_metadata: node_metadata.clone(),
        node_id,
    };

    let url = API_BASE_PATH.to_owned() + &routes::node::folder::create(root_node_id);

    let test_request = server
        .post(&url)
        .json(&create_node_request)
        .add_header("Authorization", format!("Bearer {}", jwt))
        .await;

    assert_eq!(test_request.status_code(), StatusCode::CREATED);
    let create_folder_response: PostCreateFolderResponse = test_request.json();

    let expected_response = PostCreateFolderResponse::Created(EncryptedNode {
        id: node_id,
        change_count: 0,
        parent_id: Some(root_node_id),
        owner_id: self_id,
        deleted_on: None,
        node_type: NodeType::Folder,
        current_revision: None,
        encrypted_metadata: node_metadata,
        has_access: vec![(self_id, TEST_USERNAME1.to_string())],
    });

    assert_eq!(expected_response, create_folder_response);
}

#[tokio::test]
pub async fn test_path_between_nodes() {
    let server = get_server().await;
    let (jwt, root_node_id) = login1(&server).await;
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
        API_BASE_PATH.to_owned() + &routes::node::folder::create(root_node_id);
    let create_node_request1_response = server
        .post(&create_folder_in_root_url)
        .add_header("Authorization", format!("Bearer {}", jwt))
        .json(&create_node_request1)
        .await;

    assert_eq!(
        create_node_request1_response.status_code(),
        StatusCode::CREATED
    );

    let create_folder_url =
        API_BASE_PATH.to_owned() + &routes::node::folder::create(create_node_request1.node_id);

    let create_node_request2_response = server
        .post(&create_folder_url)
        .add_header("Authorization", format!("Bearer {}", jwt))
        .json(&create_node_request2)
        .await;
    assert_eq!(
        create_node_request2_response.status_code(),
        StatusCode::CREATED
    );

    let create_node_request3_response = server
        .post(&create_folder_in_root_url)
        .add_header("Authorization", format!("Bearer {}", jwt))
        .json(&create_node_request3)
        .await;
    assert_eq!(
        create_node_request3_response.status_code(),
        StatusCode::CREATED
    );

    let path_between_nodes_url1 = API_BASE_PATH.to_owned()
        + &routes::node::path_between_nodes(root_node_id, create_node_request2.node_id);

    let path_between_nodes_response1 = server
        .get(&path_between_nodes_url1)
        .add_header("Authorization", format!("Bearer {}", jwt))
        .await;

    let path_between_nodes_url2 = API_BASE_PATH.to_owned()
        + &routes::node::path_between_nodes(
            create_node_request3.node_id,
            create_node_request2.node_id,
        );

    let path_between_nodes_response2 = server
        .get(&path_between_nodes_url2)
        .add_header("Authorization", format!("Bearer {}", jwt))
        .await;

    match path_between_nodes_response1.json::<GetPathBetweenNodesResponse>() {
        GetPathBetweenNodesResponse::Ok(path) => {
            assert_eq!(path[0].id, root_node_id);
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
pub async fn test_share() {
    let server = get_server().await;
    let (jwt1, root_node_id1) = login1(&server).await;
    let (jwt2, _root_node_id2) = login2(&server).await;

    // user 1 creates a folder
    let (folder1_id, metadata1) = {
        let id = NodeId::random();
        let node_metadata = random_metadata();

        let create_node_request1 = PostCreateFolderRequest {
            parent_metadata_version: 0,
            parent_metadata: random_metadata(),
            node_metadata: node_metadata.clone(),
            node_id: id,
        };
        let create_folder_in_root_url =
            API_BASE_PATH.to_owned() + &routes::node::folder::create(root_node_id1);
        let create_node_request1_response = server
            .post(&create_folder_in_root_url)
            .add_header("Authorization", format!("Bearer {}", jwt1))
            .json(&create_node_request1)
            .await;

        assert_eq!(
            create_node_request1_response.status_code(),
            StatusCode::CREATED
        );

        (id, node_metadata)
    };

    // user2 should have zero shared nodes
    {
        let get_accepted_shared_nodes_url =
            API_BASE_PATH.to_owned() + &routes::node::share::get_accepted_shared();
        let accepted_shared_node_response =
            auth(server.get(&get_accepted_shared_nodes_url), &jwt2).await;
        let accepted_shared_node_response_body: GetAcceptedSharedResponse =
            accepted_shared_node_response.json();
        assert_eq!(
            accepted_shared_node_response_body,
            GetAcceptedSharedResponse::Ok(vec![])
        );
    }

    // create the share "url"
    let share_id = {
        let share_node_url = API_BASE_PATH.to_owned() + &routes::node::share::share(folder1_id);
        let post_share_node_request = PostShareNodeRequest {
            wrapped_metadata_key: EncryptionKey::nil(),
        };
        let create_share_node_response = auth(server.post(&share_node_url), &jwt1)
            .json(&post_share_node_request)
            .await;
        let create_share_node_response_body: PostShareNodeResponse =
            create_share_node_response.json();

        if let PostShareNodeResponse::Ok(share_id) = create_share_node_response_body {
            share_id
        } else {
            panic!("unexpected response: {:?}", create_share_node_response);
        }
    };

    // get share url info
    {
        let get_share_info_url =
            API_BASE_PATH.to_owned() + &routes::node::share::get_share_accept_info(share_id);
        let get_share_info_response = auth(server.get(&get_share_info_url), &jwt2).await;
        let get_share_info_response_body: GetAcceptShareInfoResponse =
            get_share_info_response.json();

        if let GetAcceptShareInfoResponse::Ok(share_info) = get_share_info_response_body {
            assert_eq!(share_info.node_id, folder1_id)
        } else {
            panic!("unexpected response: {:?}", get_share_info_response);
        }
    }

    // accept share
    {
        let accept_share_url =
            API_BASE_PATH.to_owned() + &routes::node::share::accept_share(share_id);
        let accept_share_body = PostAcceptShareRequest {
            new_wrapped_metadata_key: EncryptionKey::nil(),
        };
        let accept_share_response = auth(server.post(&accept_share_url), &jwt2)
            .json(&accept_share_body)
            .await;
        let accept_share_response_body: PostAcceptShareResponse = accept_share_response.json();

        assert_eq!(PostAcceptShareResponse::Ok, accept_share_response_body);
    }

    // user 2 should have one shared item
    {
        let get_accepted_shared_nodes_url =
            API_BASE_PATH.to_owned() + &routes::node::share::get_accepted_shared();
        let accepted_shared_node_response =
            auth(server.get(&get_accepted_shared_nodes_url), &jwt2).await;
        let accepted_shared_node_response_body: GetAcceptedSharedResponse =
            accepted_shared_node_response.json();
        let GetAcceptedSharedResponse::Ok(accepted_nodes) = accepted_shared_node_response_body;
        assert_eq!(accepted_nodes.len(), 1);
        assert_eq!(accepted_nodes[0].1.id, folder1_id);
    }

    // user 2 should be able to read the shared folder
    {
        let url = API_BASE_PATH.to_owned() + &routes::node::by_id(folder1_id);

        let test_request = auth(server.get(&url), &jwt2).await;

        assert_eq!(test_request.status_code(), StatusCode::OK);
        let response: GetNodeResponse = test_request.json();

        if let GetNodeResponse::Ok(node) = response {
            assert_eq!(node.encrypted_metadata, metadata1)
        } else {
            panic!("unexpected response: {:?}", response);
        }
    }

    // user 2 should be able to add a folder to the shared folder
    let (folder2_id, folder2_metadata) = {
        let folder2_id = NodeId::random();
        let folder2_metadata = random_metadata();
        let create_node_request = PostCreateFolderRequest {
            parent_metadata_version: 0,
            parent_metadata: random_metadata(),
            node_metadata: folder2_metadata.clone(),
            node_id: folder2_id,
        };

        let url = API_BASE_PATH.to_owned() + &routes::node::folder::create(folder1_id);

        let test_request = auth(server.post(&url), &jwt2)
            .json(&create_node_request)
            .await;

        assert_eq!(test_request.status_code(), StatusCode::CREATED);
        let _create_folder_response: PostCreateFolderResponse = test_request.json();

        (folder2_id, folder2_metadata)
    };

    // user1 should see all folders
    {
        let url = API_BASE_PATH.to_owned() + &routes::node::accessible_path(folder2_id);
        let test_request: GetAccessiblePathResponse = auth(server.get(&url), &jwt1).await.json();

        if let GetAccessiblePathResponse::Ok(path) = test_request {
            assert_eq!(path.len(), 3);
            assert_eq!(path[0].id, root_node_id1);
            assert_eq!(path[1].id, folder1_id);
            assert_eq!(path[2].id, folder2_id);
        } else {
            panic!("unexpected response: {:?}", test_request);
        }
    }

    // user2 should see the last two folders
    {
        let url = API_BASE_PATH.to_owned() + &routes::node::accessible_path(folder2_id);
        let test_request: GetAccessiblePathResponse = auth(server.get(&url), &jwt2).await.json();

        if let GetAccessiblePathResponse::Ok(path) = test_request {
            assert_eq!(path.len(), 2);
            assert_eq!(path[0].id, folder1_id);
            assert_eq!(path[1].id, folder2_id);
        } else {
            panic!("unexpected response: {:?}", test_request);
        }
    }

    // user 1 should be able to read the folder that was created in his folder
    {
        let url = API_BASE_PATH.to_owned() + &routes::node::by_id(folder2_id);

        let test_request = auth(server.get(&url), &jwt1).await;

        assert_eq!(test_request.status_code(), StatusCode::OK);
        let response: GetNodeResponse = test_request.json();

        if let GetNodeResponse::Ok(node) = response {
            assert_eq!(node.encrypted_metadata, folder2_metadata)
        } else {
            panic!("unexpected response: {:?}", response);
        }
    }
}

fn auth(request: TestRequest, jwt: &String) -> TestRequest {
    request.add_header("Authorization", format!("Bearer {}", jwt))
}

#[tokio::test]
pub async fn test_file() {
    let server = get_server().await;
    let (jwt, root_node_id) = login1(&server).await;

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

    let create_file_url = API_BASE_PATH.to_owned() + &routes::node::file::create(root_node_id);

    let test_request = server
        .post(&create_file_url)
        .add_header("Authorization", format!("Bearer {}", jwt))
        .json(&create_node_request)
        .await;

    assert_eq!(test_request.status_code(), StatusCode::CREATED);
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
        + &routes::node::chunks(create_node_request.node_id, revision.id, 1);
    let chunk_url2 = API_BASE_PATH.to_owned()
        + &routes::node::chunks(create_node_request.node_id, revision.id, 2);

    let chunk1_response = server
        .post(&chunk_url1)
        .add_header("Authorization", format!("Bearer {}", jwt))
        .bytes(chunk1.clone())
        .await;
    assert_eq!(chunk1_response.status_code(), StatusCode::CREATED);

    let chunk2_response = server
        .post(&chunk_url2)
        .add_header("Authorization", format!("Bearer {}", jwt))
        .bytes(chunk2.clone())
        .await;
    assert_eq!(chunk2_response.status_code(), StatusCode::CREATED);

    let commit_file_url = API_BASE_PATH.to_owned()
        + &routes::node::file::commit(create_node_request.node_id, revision.id);

    let commit_file_response: PostCommitFileResponse = server
        .post(&commit_file_url)
        .add_header("Authorization", format!("Bearer {}", jwt))
        .await
        .json();

    let node;
    match commit_file_response {
        PostCommitFileResponse::Ok(_node) => node = _node,
        _ => {
            panic!("failed to commit file")
        }
    }

    assert_eq!(node.current_revision.as_ref().unwrap().id, revision.id);

    let get_node_url = API_BASE_PATH.to_owned() + &routes::node::by_id(create_node_request.node_id);

    let node_response: GetNodeResponse = server
        .get(&get_node_url)
        .add_header("Authorization", format!("Bearer {}", jwt))
        .await
        .json();

    match node_response {
        GetNodeResponse::Ok(same_node_as_before) => {
            assert_eq!(same_node_as_before, node)
        }
        _ => {
            panic!("failed to get node")
        }
    }

    let get_chunk_response1 = server
        .get(&chunk_url1)
        .add_header("Authorization", format!("Bearer {}", jwt))
        .await;
    let get_chunk_response2 = server
        .get(&chunk_url2)
        .add_header("Authorization", format!("Bearer {}", jwt))
        .await;

    assert_eq!(get_chunk_response1.status_code(), StatusCode::OK);
    assert_eq!(get_chunk_response2.status_code(), StatusCode::OK);

    let chunk1_bytes_response = get_chunk_response1.as_bytes();
    let chunk2_bytes_response = get_chunk_response2.as_bytes();

    assert_eq!(chunk1, chunk1_bytes_response);
    assert_eq!(chunk2, chunk2_bytes_response);

    // try to commit the file a second time
    let commit_file_response2: PostCommitFileResponse = server
        .post(&commit_file_url)
        .add_header("Authorization", format!("Bearer {}", jwt))
        .await
        .json();
    let expected_commit_file_response2 = PostCommitFileResponse::BadRequest(AlreadyCommitted);
    assert_eq!(commit_file_response2, expected_commit_file_response2);
}

pub async fn get_server() -> TestServer {
    let config = AppConfig::load(&PathBuf::from("./crabdrive.toml")).unwrap();

    // https://stackoverflow.com/questions/58649529/how-to-create-multiple-memory-databases-in-sqlite3
    let db_path = format!("file:{}?mode=memory&cache=shared", UUID::random());

    let pool = create_pool(&db_path, config.db.pool_size);

    let vfs = Sfs::new(&config.storage.dir);

    let keys = Keys::new(&config.auth.jwt_secret);

    let node_repository = NodeState::new(Arc::new(pool.clone()));
    let revision_repository = RevisionService::new(Arc::new(pool.clone()));
    let user_repository = UserState::new(Arc::new(pool.clone()), keys.clone());
    let share_repository = ShareRepositoryImpl::new(Arc::new(pool.clone()));

    let state = AppState::new(
        config.clone(),
        pool,
        vfs,
        node_repository,
        revision_repository,
        user_repository,
        share_repository,
        keys,
    );

    prepare_db(&state);

    let app = Router::<AppState>::new()
        .merge(crate::http::routes::routes())
        .with_state(state.clone())
        .layer(middleware::from_fn(logging_middleware))
        .layer(CatchPanicLayer::custom(crate::http::server::handle_panic));

    let server = TestServer::new(app).unwrap();

    let register_user1_response_body = PostRegisterRequest {
        username: TEST_USERNAME1.to_string(),
        password: TEST_USERNAME1.to_string(),
        keys: UserKeys::nil(),
    };

    let register_response1 = server
        .post(ROUTE_REGISTER)
        .json(&register_user1_response_body)
        .await;

    assert_eq!(register_response1.status_code(), StatusCode::CREATED);

    let register_user2_response_body = PostRegisterRequest {
        username: TEST_USERNAME2.to_string(),
        password: TEST_USERNAME2.to_string(),
        keys: UserKeys::nil(),
    };

    let register_response2 = server
        .post(ROUTE_REGISTER)
        .json(&register_user2_response_body)
        .await;

    assert_eq!(register_response2.status_code(), StatusCode::CREATED);

    server
}

// copied from server.rs/start
fn prepare_db(state: &AppState) {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./res/migrations/");
    {
        let mut conn = state.db_pool.get().unwrap();
        conn.run_pending_migrations(MIGRATIONS).unwrap();
    }
}

pub async fn login1(server: &TestServer) -> (String, NodeId) {
    login(server, TEST_USERNAME1.to_string()).await
}

pub async fn login2(server: &TestServer) -> (String, NodeId) {
    login(server, TEST_USERNAME2.to_string()).await
}

pub async fn login(server: &TestServer, username: String) -> (String, NodeId) {
    let login = PostLoginRequest {
        username: username.clone(),
        password: username,
    };

    let login_url = API_BASE_PATH.to_owned() + ROUTE_LOGIN;

    let login_request = server.post(&login_url).json(&login).await;

    let login_response: PostLoginResponse = login_request.json();

    if let PostLoginResponse::Ok(response) = login_response {
        (response.bearer_token, response.root_node_id)
    } else {
        panic!("login failed: {:?}", login_response);
    }
}
