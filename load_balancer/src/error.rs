use common::error::CommonError;
use thiserror::Error;
//
#[derive(Error, Debug)]
pub enum LoadBalancerError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("CommonError: {0}")]
    CommonError(#[from] CommonError),
}
