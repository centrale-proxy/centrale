use std::error::Error as StdError;
use thiserror::Error;

//
#[derive(Error, Debug)]
pub enum SampleServerError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Error: {0}")]
    StringError(String),

    #[error("Std error: {0}")]
    StdError(#[from] Box<dyn StdError>),

    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
}
