use std::error::Error as StdError;
use thiserror::Error;

//
#[derive(Error, Debug)]
pub enum CentraleError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Error: {0}")]
    StringError(String),

    #[error("Std error: {0}")]
    StdError(#[from] Box<dyn StdError>),

    #[error("Serde JSON error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Actix Web error: {0}")]
    ActixWebError(#[from] actix_web::error::Error),

    #[error("No DB error")]
    NoDb,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Missing subdomain")]
    MissingSubdomain,

    #[error("Invalid subdomain")]
    InvalidSubdomain,
}
