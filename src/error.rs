use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("failed to start")]
    FailedToStart(#[from] std::io::Error),
    #[error("Failed to read environment variables")]
    InvalidEnvironmentVariables(#[from] envy::Error),
    #[error("Invalid Key: {0}")]
    InvalidKey(#[from] jsonwebtoken::errors::Error),
    #[error("Failed to KeySet")]
    InvalidKeySet,
    #[error("Invalid Encode: {0}")]
    InvalidEncode(jsonwebtoken::errors::Error),
    #[error("Invalid Decode: {0}")]
    InvalidDecode(jsonwebtoken::errors::Error),
}
