use tracing_subscriber::EnvFilter;

#[derive(Debug, thiserror::Error)]
pub enum LoggingError {
    #[error("invalid log filter: {0}")]
    InvalidFilter(String),
}

pub fn init(log_level: &str) -> Result<(), LoggingError> {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(log_level))
        .map_err(|err| LoggingError::InvalidFilter(err.to_string()))?;

    tracing_subscriber::fmt().with_env_filter(filter).init();
    Ok(())
}
