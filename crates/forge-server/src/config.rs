use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_path: PathBuf,
    pub bind_addr: SocketAddr,
    pub log_level: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("invalid FORGE_BIND_ADDR `{value}`: {source}")]
    InvalidBindAddr {
        value: String,
        #[source]
        source: std::net::AddrParseError,
    },
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let database_path = std::env::var("FORGE_DATABASE_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("forge.db"));

        let bind_value =
            std::env::var("FORGE_BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
        let bind_addr = bind_value
            .parse()
            .map_err(|source| ConfigError::InvalidBindAddr {
                value: bind_value,
                source,
            })?;

        let log_level = std::env::var("FORGE_LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

        Ok(Self {
            database_path,
            bind_addr,
            log_level,
        })
    }
}
