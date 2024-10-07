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
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        status.into_response()
    }
}
