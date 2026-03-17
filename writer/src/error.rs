use r2d2_sqlite::rusqlite;
// use std::{error::Error as StdError, str::Utf8Error};
use thiserror::Error;

//
#[derive(Error, Debug)]
pub enum WriterError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
    #[error("Downstream closed")]
    DownstreamClosed,
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}
