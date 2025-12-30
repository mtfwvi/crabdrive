use std::env;

#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    Dev,
    Prod,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub env: Environment,
    pub host: String,
    pub port: u16,
    pub log_dir: String,
}

impl Config {
    pub fn load() -> Self {
        let _ = dotenvy::dotenv();

        let environment = env::var("CRABDRIVE_ENV")
            .unwrap_or_else(|_| {
                if cfg!(debug_assertions) {
                    "DEV".to_string()
                } else {
                    "PROD".to_string()
                }
            })
            .to_lowercase();

        let addr = env::var("CRABDRIVE_ADDR").unwrap_or("127.0.0.1:2722".to_string());

        let Some((host, port)) = addr.rsplit_once(':') else {
            panic!("Invalid format!")
        };

        let log_dir = env::var("CRABDRIVE_LOG_DIR").unwrap_or("/var/log/crabdrive/".to_string());
        let log_path = std::path::Path::new(&log_dir);

        if !log_path.exists() || !log_path.is_dir() {
            panic!("Misconfigured log directory!");
        }

        Config {
            env: if environment == "dev" {
                Environment::Dev
            } else {
                Environment::Prod
            },
            host: host.to_string(),
            port: port.parse::<u16>().unwrap(),
            log_dir,
        }
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn is_dev(&self) -> bool {
        self.env == Environment::Dev
    }

    pub fn is_prod(&self) -> bool {
        self.env == Environment::Prod
    }
}
