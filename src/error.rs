use axum::response::IntoResponse;
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("failed to start")]
    FailedToStart(#[from] std::io::Error),
    #[error("Invalid Format: {0}")]
    InvalidFormat(#[from] serde_urlencoded::ser::Error),
    #[error("Invalid Request: {0}")]
    InvalidRequest(#[from] reqwest::Error),
    #[error("InvalidEnvironmentVariable: {0}")]
    InvalidEnvironmentVariable(#[from] envy::Error),
    #[error("InvalidRequest")]
    InvalidRequestFormat,
    #[error("Database migration failed: {0}")]
    InvalidDatabaseMigration(#[from] sqlx::migrate::MigrateError),
    #[error("failed in database: {0}")]
    InvalidDatabase(#[from] sqlx::Error),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Self::InvalidRequestFormat => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        status.into_response()
    }
}
