#![allow(dead_code)] // TODO: Remove before submission
mod http;
mod storage;
mod user;

use tracing::{debug, trace};
use tracing_subscriber::{
    Layer, Registry, filter::LevelFilter, layer::SubscriberExt, util::SubscriberInitExt,
};

use crate::http::{AppConfig, server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::load()?;

    if config.log.minimum_level != "NONE" {
        let layers: Vec<Box<dyn Layer<Registry> + Send + Sync>> = config
            .log
            .targets
            .iter()
            .filter_map(|target| {
                let path = std::path::Path::new(target);

                return match target.as_str() {
                    ":stdout:" => Some(
                        tracing_subscriber::fmt::layer()
                            .with_ansi(true)
                            .with_writer(std::io::stdout)
                            .boxed(),
                    ),
                    ":stderr:" => Some(
                        tracing_subscriber::fmt::layer()
                            .with_ansi(true)
                            .with_writer(std::io::stderr)
                            .boxed(),
                    ),

                    t if t.ends_with(std::path::MAIN_SEPARATOR) => {
                        if !path.exists() {
                            eprintln!("Error: Log folder does not exist: {}", path.display());
                            std::process::exit(-1);
                        }
                        let appender = tracing_appender::rolling::daily(path, "crabdrive-server");
                        Some(
                            tracing_subscriber::fmt::layer()
                                .with_ansi(false)
                                .with_writer(appender)
                                .boxed(),
                        )
                    }

                    _ => {
                        let file = std::fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(path)
                            .ok()?;

                        let layer = tracing_subscriber::fmt::layer()
                            // This is currently broken, for some reason it still emits ANSI Codes
                            // for Bold / Italic Text.
                            //   -> See also https://github.com/tokio-rs/tracing/issues/3116
                            // A temporary workaround might be to sort the layers first (paths first,
                            // then console)
                            .with_ansi(false)
                            .with_writer(file)
                            .compact();

                        if target.ends_with(".json") {
                            Some(layer.json().boxed())
                        } else {
                            Some(layer.boxed())
                        }
                    }
                };
            })
            .collect();

        tracing_subscriber::registry()
            .with(layers)
            .with(match config.log.minimum_level.as_str() {
                "ERROR" => LevelFilter::ERROR,
                "WARN" => LevelFilter::WARN,
                "INFO" => LevelFilter::INFO,
                "DEBUG" => LevelFilter::DEBUG,
                "TRACE" => LevelFilter::TRACE,
                _ => unreachable!("How the hell did you do that?"),
            })
            .init();
    }

    trace!("\n{}", config);

    let _ = server::start(config).await;

    Ok(())
}
