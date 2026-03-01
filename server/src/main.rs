mod db;
mod http;
mod request_handler;
mod storage;
mod user;

#[cfg(test)]
mod test;

use clap::{Arg, Command, crate_version, value_parser};
use tracing::trace;
use tracing_subscriber::{filter::LevelFilter, layer::SubscriberExt, util::SubscriberInitExt};

use std::io::Write;

use crate::http::{AppConfig, server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("starting crabdrive");
    // Get CLI arguments & flags
    let matches = Command::new("crabdrive-server")
        .about("Starts the crabdrive server")
        .version(crate_version!())
        .arg(
            Arg::new("config")
                .short('C')
                .long("config")
                .required(false)
                .value_name("FILE")
                .help("Sets a custom configuration file.")
                .default_value("./crabdrive.toml")
                .value_parser(value_parser!(std::path::PathBuf)),
        )
        .arg(
            Arg::new("template")
                .long("config-template")
                .required(false)
                .value_name("FILE")
                .help("Generates a default configuration template at the given path.")
                .value_parser(value_parser!(std::path::PathBuf)),
        )
        .get_matches();

    if let Some(template_path) = matches.get_one::<std::path::PathBuf>("template") {
        // Generate a default configuration template
        let mut format_options = confique::toml::FormatOptions::default();
        format_options.general.include_default_or_required_comment = false;

        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(template_path)?;
        file.write_all(confique::toml::template::<AppConfig>(format_options).as_bytes())?;

        println!(
            "Created an example config here: {}",
            &template_path.display()
        );
        return Ok(());
    }

    let config = AppConfig::load(matches.get_one::<std::path::PathBuf>("config").unwrap())?;

    if config.log.minimum_level != "NONE" {
        let layers = config.parse_tracing_layers()?;

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
