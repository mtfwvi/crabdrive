# State

Shared server state is managed by Axum. It lets all request handlers "extract" states from the Router.

## Usage

```rust
use crate::http::AppState;
use axum::{extract::State, response::Json};
use serde_json::{json, Value};

async fn health_handler(
    State(state): State<AppState>
) -> Json<Value> {
    // Acess configuration values
    let environment = &state.config.environment;

    Json(json!({
        "status": "active",
        "env": environment
    }))
}
```

## Adding new fields

- For Immutable state (like DB Pools): Use `Arc<T>` [^1]
- For Mutable state (like Counters): Use `Arc<Mutex<T>>` or `Arc<RwLock<T>>`
- States must be: `Send + Sync + 'static`
- States must be cloneable (and cheap to clone), as the state is cloned on each new request!

[^1]: Some DB pools already have a built-in Arc, wrapping is in this case not needed.