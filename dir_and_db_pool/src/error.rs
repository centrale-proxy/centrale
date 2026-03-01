use thiserror::Error;

#[derive(Debug, Error)]
pub enum DirsqlError {
    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("String error: {0}")]
    StringError(String),
}
