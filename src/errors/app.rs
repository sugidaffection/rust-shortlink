use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Failed to load .env file: {0}")]
    EnvLoadError(#[from] dotenvy::Error),

    #[error("Database migration failed: {0}")]
    MigrationError(String),

    #[error("Failed to start HTTP server: {0}")]
    ServerError(#[from] std::io::Error),
}
