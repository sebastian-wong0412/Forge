use forge_server::api;
use forge_server::config::Config;
use forge_server::{db, logging};

#[derive(Debug, thiserror::Error)]
enum MainError {
    #[error(transparent)]
    Config(#[from] forge_server::config::ConfigError),
    #[error(transparent)]
    Logging(#[from] forge_server::logging::LoggingError),
    #[error(transparent)]
    Db(#[from] forge_server::db::DbError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), MainError> {
    let _ = dotenvy::dotenv();
    let config = Config::from_env()?;
    logging::init(&config.log_level)?;

    let pool = db::connect(&config.database_path).await?;
    db::migrate(&pool).await?;

    let app = api::router(pool);
    let listener = tokio::net::TcpListener::bind(config.bind_addr).await?;
    tracing::info!(addr = %config.bind_addr, "forge listening");
    axum::serve(listener, app).await?;
    Ok(())
}
