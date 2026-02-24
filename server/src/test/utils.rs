use axum_test::TestServer;
use crabdrive_common::storage::NodeType;

use crate::{http::AppState, user::persistence::model::user_entity::UserEntity};

struct TestFactory {

}

struct TestContext {
    /// The test server to make requests against
    pub server: TestServer,
    pub state: AppState,
    /// All pre-registered users
    pub users: Vec<TestUser>
}

struct TestUser {
    pub entity: UserEntity
}

struct TestNode {
    pub node_type: NodeType,
    pub children:
}

pub async fn get_server() -> TestServer {
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
        .merge(crate::http::routes::routes())
        .with_state(state.clone())
        .layer(middleware::from_fn(logging_middleware))
        .layer(CatchPanicLayer::custom(crate::http::server::handle_panic));

    let server = TestServer::new(app).unwrap();

    let register_admin_request_body = PostRegisterRequest {
        username: TEST_USERNAME.to_string(),
        password: TEST_PASSWORD.to_string(),
        keys: UserKeys::nil(),
    };

    let register_response = server
        .post(ROUTE_REGISTER)
        .json(&register_admin_request_body)
        .await;

    assert_eq!(register_response.status_code(), StatusCode::CREATED);

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
