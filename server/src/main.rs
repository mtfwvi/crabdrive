use axum::{
    routing::get,
    Router
};
use tokio::signal;

async fn cc_signal() {
    let _ = signal::ctrl_c().await;
    println!("Exiting");
}

#[tokio::main]
async fn main() {
    const ADDR: &'static str = "127.0.0.1:2722";

    let app = Router::new().route("/", get(|| async { "Hello Crabdrive!" }));
    if let Ok(listener) =  tokio::net::TcpListener::bind(ADDR).await {
        println!("Server running on http://{ADDR}");
        axum::serve(listener, app)
            .with_graceful_shutdown(cc_signal())
            .await.unwrap();
    } else {
        panic!("Cannot bind TcpListener")
    }
}
