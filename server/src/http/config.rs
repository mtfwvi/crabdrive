use confique::Config;
use serde::{Deserialize, Serialize};
use tracing_subscriber::{Layer, Registry};

use crate::http::config::confique_database_config_layer::DatabaseConfigLayer;
use crate::http::config::confique_log_config_layer::LogConfigLayer;
use crate::http::config::confique_sever_config_layer::SeverConfigLayer;
use crate::http::config::confique_storage_config_layer::StorageConfigLayer;

fn is_valid_log_level(level: &String) -> Result<(), String> {
    let level: &str = level.as_ref();
    match level {
        "TRACE" | "DEBUG" | "INFO" | "WARN" | "ERROR" | "NONE" => Ok(()),
        _ => Err("Invalid log level".to_string()),
    }
}

fn parse_list(s: &str) -> Result<Vec<String>, std::convert::Infallible> {
    Ok(s.split(',').map(|item| item.trim().to_string()).collect())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Environment {
    Dev,
    Prod,
}

#[derive(Debug, Clone, Serialize, Deserialize, Config)]
/// The server ncan be configured via TOML config files or environment variables.
pub struct AppConfig {
    /// The environment the application runs in.
    /// If this is set to DEV, the application may f.e. log sensitive information.
    ///
    /// **Possible values**:
    ///  - `DEV`
    ///  - `PROD`
    ///
    /// **Default**: Derived from Build Type
    #[config(env = "CRABDRIVE_ENV")]
    pub env: Environment,
    #[config(nested)]
    pub server: SeverConfig,
    #[config(nested)]
    pub db: DatabaseConfig,
    #[config(nested)]
    pub storage: StorageConfig,
    #[config(nested)]
    pub log: LogConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Config)]
pub struct SeverConfig {
    /// The address the TCP listener binds to. Can be a IPv4 or IPv6.
    ///
    /// **Default**: `127.0.0.1`
    #[config(env = "CRABDRIVE_ADDR")]
    pub address: std::net::IpAddr,
    /// The port the TCP listener binds to.
    ///
    /// **Default**: `2722`
    #[config(env = "CRABDRIVE_PORT")]
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, Config)]
pub struct DatabaseConfig {
    /// The path to the database file. It can be one of the following formats:
    /// - `/path/to/db.sqlite` or `file:///path/to/db.sqlite`
    /// - `:memory:`
    ///
    /// **Notes**: If the file does not exist, it is created.
    ///
    /// **Default**: `:memory:`
    #[config(env = "CRABDRIVE_DB_PATH")]
    pub path: String,
    /// Number of connections opened to the database and stored in a connection pool.
    ///
    /// **Notes**: This will open a corresponding number of file handles.
    ///
    /// **Default**: 15
    #[config(env = "CRABDRIVE_DB_POOLSIZE")]
    pub pool_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Config)]
pub struct LogConfig {
    /// The minimum log level for log messages. All messages below this level will be discarded.
    /// If this is set to `None`, nothing will be logged. Possible values are:
    /// - `NONE`
    /// - `TRACE`
    /// - `DEBUG`
    /// - `INFO`
    /// - `WARN`
    /// - `ERROR`
    ///
    /// **Default**: `TRACE` when `ENV` is set to `DEV`, otherwise `WARN`
    #[config(env = "CRABDRIVE_MINIMUM_LOG_LEVEL", validate = is_valid_log_level)]
    pub minimum_level: String,

    /// The targets, where logs are piped into. If `env` is set to `DEV` or
    /// `logs.minimum_level` is set to `NONE`, this is ignored. It may be one of the following
    /// formats:
    /// - `/path/to/directory/` (note the trailing slash!) creates daily logs inside the folder
    /// - `/path/to/my_log.log` or `/path/to/my_log.json` writes into a file
    /// - `:stdout:` or `:stderr:`
    ///
    /// **Notes**: The directory is not automatically created.
    ///
    /// **Default**: `[":stdout:"]` when `env` is set to `DEV`, otherwise `["/var/log/crabdrive/"]`
    #[config(env = "CRABDRIVE_LOG_TARGETS", parse_env = parse_list)]
    pub targets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Config)]
pub struct StorageConfig {
    /// The path to the storage directory. Can be of the following formats:
    /// - `/path/to/directory/`
    /// - `:memory:`
    ///
    /// **Notes**: The directory is not automatically created.
    ///
    /// **Default**: `:memory:`
    #[config(env = "CRABDRIVE_STORAGE_DIR")]
    pub dir: String,

    /// The storage limit for ALL files, in Bytes.
    ///
    /// **Notes**: When [`AppConfig::storage_dir`] is set to `:memory:`, this will limit the memory
    ///            used by the application for storage.
    ///
    /// **Default**: `500_000_000` (500MB)
    #[config(env = "CRABDRIVE_STORAGE_LIMIT")]
    pub limit: usize,
}

type ConfLayer = <AppConfig as Config>::Layer;
type ParsedLayerVecResult =
    Result<Vec<Box<dyn Layer<Registry> + Send + Sync>>, Box<dyn std::error::Error>>;
type ParsedLayerResult = Result<Box<dyn Layer<Registry> + Send + Sync>, Box<dyn std::error::Error>>;

impl AppConfig {
    pub fn default_values() -> ConfLayer {
        // This is a bit "hacky" implementation, bacause confique does not allow for runtime-generated
        // defaults
        ConfLayer {
            env: if cfg!(debug_assertions) {
                Some(Environment::Dev)
            } else {
                Some(Environment::Prod)
            },
            server: SeverConfigLayer {
                address: Some("127.0.0.1".parse().unwrap()),
                port: Some(2722),
            },
            db: DatabaseConfigLayer {
                path: Some(":memory:".into()),
                pool_size: Some(15),
            },
            storage: StorageConfigLayer {
                dir: Some(":memory:".into()),
                limit: Some(500_000_000),
            },
            log: LogConfigLayer {
                minimum_level: Some(if cfg!(debug_assertions) {
                    "TRACE".to_string()
                } else {
                    "WARN".to_string()
                }),
                targets: Some(if cfg!(debug_assertions) {
                    vec![":stdout:".to_string()]
                } else {
                    vec!["/var/log/crabdrive/".to_string()]
                }),
            },
        }
    }

    pub fn load(configfile: &std::path::PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();

        let config = AppConfig::builder()
            .env()
            .file(configfile)
            .preloaded(AppConfig::default_values())
            .load()?;

        Ok(config)
    }

    /// Parses all targets (in [`AppConfig::log::targets`]) into Tracing Layers
    pub fn parse_tracing_layers(&self) -> ParsedLayerVecResult {
        self.log
            .targets
            .iter()
            .map(|target| -> ParsedLayerResult {
                let path = std::path::Path::new(target);

                match target.as_str() {
                    // `:stdout:` writes to the standard output
                    ":stdout:" => Ok(tracing_subscriber::fmt::layer()
                        .with_ansi(true)
                        .with_writer(std::io::stdout)
                        .boxed()),
                    // `:stderr:` writes to the standard error output
                    ":stderr:" => Ok(tracing_subscriber::fmt::layer()
                        .with_ansi(true)
                        .with_writer(std::io::stderr)
                        .boxed()),
                    // If the target ends with a path seperator, it's expected to be a folder.
                    t if t.ends_with(std::path::MAIN_SEPARATOR) => {
                        if !path.exists() {
                            // Do not automatically create folders
                            return Err(std::io::Error::from(std::io::ErrorKind::NotFound).into());
                        }
                        let appender = tracing_appender::rolling::daily(path, "crabdrive-server");
                        Ok(tracing_subscriber::fmt::layer()
                            .with_ansi(false)
                            .with_writer(appender)
                            .boxed())
                    }
                    // Everything else is assumed to be a file
                    _ => {
                        let file = std::fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(path)?;

                        let layer = tracing_subscriber::fmt::layer()
                            // This is currently broken, for some reason it still emits ANSI Codes
                            // for Bold / Italic Text.
                            //   -> See also https://github.com/tokio-rs/tracing/issues/3116
                            // A temporary workaround might be to sort the layers first (paths first,
                            // then console)
                            .with_ansi(false)
                            .with_writer(file)
                            .compact();

                        if path.extension() == Some(std::ffi::OsStr::new("json")) {
                            Ok(layer.json().boxed())
                        } else {
                            Ok(layer.boxed())
                        }
                    }
                }
            })
            .collect()
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.server.address, self.server.port)
    }

    pub fn is_dev(&self) -> bool {
        self.env == Environment::Dev
    }

    pub fn is_prod(&self) -> bool {
        self.env == Environment::Prod
    }
}

impl std::fmt::Display for AppConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Loaded Configuration: ({} Bytes)", size_of::<Self>())?;
        writeln!(f, "├── Environment:   {:?}", self.env)?;
        writeln!(f, "├─┬ Server:")?;
        writeln!(f, "│ └── Address:     {}", self.addr())?;
        writeln!(f, "├─┬ Database:")?;
        writeln!(f, "│ ├── Path:        {}", self.db.path)?;
        writeln!(f, "│ └── Pool Size:   {}", self.db.pool_size)?;
        writeln!(f, "├─┬ Storage:")?;
        writeln!(f, "│ ├── Directory:   {}", self.storage.dir)?;
        writeln!(f, "│ └── Limit:       {} bytes", self.storage.limit)?;
        writeln!(f, "└─┬ Logging:")?;
        writeln!(f, "  ├── Min Level:   {}", self.log.minimum_level)?;
        writeln!(f, "  └── Targets:     {:?}", self.log.targets)?;
        Ok(())
    }
}
