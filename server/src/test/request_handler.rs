use crate::db::connection::create_pool;
use crate::http::middleware::logging_middleware;
use crate::http::{AppConfig, AppState, routes};
use crate::storage::node::persistence::model::node_entity::NodeEntity;
use crate::storage::node::persistence::node_repository::NodeState;
use crate::storage::revision::persistence::revision_repository::RevisionService;
use crate::storage::vfs::backend::Sfs;
use crate::user::persistence::model::encryption_key::EncryptionKey;
use crate::user::persistence::model::user_entity::UserEntity;
use axum::{Router, middleware};
use axum_test::TestServer;
use chrono::Local;
use crabdrive_common::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::iv::IV;
use crabdrive_common::payloads::node::request::file::PostCreateFileRequest;
use crabdrive_common::payloads::node::request::folder::PostCreateFolderRequest;
use crabdrive_common::payloads::node::response::file::PostCreateFileResponse;
use crabdrive_common::payloads::node::response::folder::PostCreateFolderResponse;
use crabdrive_common::routes::{CREATE_FILE_ROUTE, CREATE_FOLDER_ROUTE};
use crabdrive_common::storage::NodeType;
use crabdrive_common::uuid::UUID;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use formatx::formatx;
use rand::RngCore;
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
pub async fn test_create_folder() {
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

    match create_folder_response {
        PostCreateFolderResponse::Created(ref node) => {
            assert_eq!(node.encrypted_metadata, node_metadata);
            assert_eq!(node.id, node_id);
            assert_eq!(node.current_revision, None);
            assert_eq!(node.node_type, NodeType::Folder);
        }
        _ => {
            panic!()
        }
    }

    //TODO query node
}

#[tokio::test]
pub async fn test_create_file() {
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

    let url = API_BASE_PATH.to_owned() + &formatx!(CREATE_FILE_ROUTE, UUID::nil()).unwrap();

    let test_request = server.post(&url).json(&create_node_request).await;

    let create_folder_response: PostCreateFileResponse = test_request.json();

    match create_folder_response {
        PostCreateFileResponse::Created(ref node) => {
            assert_eq!(node.encrypted_metadata, node_metadata);
            assert_eq!(node.id, node_id);
            assert_eq!(node.node_type, NodeType::File);
            assert!(node.current_revision.is_some());
        }
        _ => {
            panic!()
        }
    }

    //TODO upload chunks
    //TODO commit file
    //TODO query node
    //TODO download chunks
}

pub fn get_server() -> TestServer {
    let config = AppConfig::load(&PathBuf::from("./crabdrive.toml")).unwrap();

    let pool = create_pool(&config.db.path, config.db.pool_size);

    let vfs = Sfs::new(&config.storage.dir);

    let node_repository = NodeState::new(Arc::new(pool.clone()));
    let revision_repository = RevisionService::new(Arc::new(pool.clone()));

    let state = AppState::new(
        config.clone(),
        pool,
        vfs,
        node_repository,
        revision_repository,
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
            storage_limit: crabdrive_common::da!(500 MB),
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
            node_type: crabdrive_common::storage::NodeType::Folder,
        };

        crate::db::operations::insert_node(&state.db_pool, &node, &EncryptedMetadata::nil())
            .unwrap();
    }
}
