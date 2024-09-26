use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("failed to start")]
    FailedToStart(#[from] std::io::Error),
}
