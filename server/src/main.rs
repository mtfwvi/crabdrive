#![allow(dead_code)] // TODO: Remove before submission
mod http;
mod storage;
mod user;

use crate::http::{Config, server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load();

    if config.is_dev() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();
    } else {
        let file_appender =
            tracing_appender::rolling::daily(&config.log_dir, "crabdrive-server.log");
        tracing_subscriber::fmt().with_writer(file_appender).init();
    }

    let _ = server::start(config).await;

    Ok(())
}
