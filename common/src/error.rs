use std::str::Utf8Error;
use thiserror::Error;

//
#[derive(Error, Debug)]
pub enum CommonError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Error: {0}")]
    StringError(String),

    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] Utf8Error),
}
