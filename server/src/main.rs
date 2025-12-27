#![allow(dead_code)] // TODO: Remove before submission
mod http;
mod storage;
mod user;

use crate::http::Config;
use crate::http::server::serve;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Use different subcribers, when in devcontainer or release
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let config = Config::load();
    let _ = serve(config).await;

    Ok(())
}
