use r2d2_sqlite::rusqlite;
// use std::{error::Error as StdError, str::Utf8Error};
use thiserror::Error;

//
#[derive(Error, Debug)]
pub enum TestSuiteError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
}
