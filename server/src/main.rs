mod storage;
mod user;

use axum::{Router, routing::get};
use tokio::signal;

async fn cc_signal() {
    let _ = signal::ctrl_c().await;
    println!("Exiting");
}

#[tokio::main]
async fn main() {
    let addr = std::env::var("CRABDRIVE_ADDR").unwrap_or("127.0.0.1:2722".to_string());

    let app = Router::new().route("/", get(|| async { "Hello Crabdrive!" }));
    if let Ok(listener) = tokio::net::TcpListener::bind(&addr).await {
        println!("Server running on http://{}", &addr);
        axum::serve(listener, app)
            .with_graceful_shutdown(cc_signal())
            .await
            .unwrap();
    } else {
        panic!("Cannot bind TcpListener. Is the port in use?")
    }
}
