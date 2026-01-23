use thiserror::Error;

#[derive(Debug, Error)]
pub enum EasyHttpMockError {
    #[error("Server error: {0}")]
    Server(#[from] ServerError),
}

#[derive(Debug, Clone, Error, PartialEq)]
pub enum ServerError {
    #[error("Server start error: {0}")]
    Start(String),
    #[error("Server stop error: {0}")]
    Stop(String),
}
