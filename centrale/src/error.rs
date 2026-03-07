use actix_http::header::ToStrError;
use r2d2::Error as R2d2Error;
use r2d2_sqlite::rusqlite;
use std::{error::Error as StdError, str::Utf8Error};
use thiserror::Error;
use url::ParseError;

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

    #[error("Missing host")]
    MissingHost,

    #[error("Unable to parse URL")]
    UnableToParseUrl,

    #[error("Invalid subdomain")]
    InvalidSubdomain,

    #[error("No host or referer present")]
    NoHostNoReferer,

    #[error("URL parse error: {0}")]
    UrlParseError(#[from] ParseError),

    #[error("No token or cookie present")]
    NoTokenOrCookiePresent,

    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("Database pool error: {0}")]
    PoolError(#[from] R2d2Error),

    #[error("Such user exists")]
    SuchUserExists,

    #[error("Unable to hash")]
    UnableToHash,
    //#[error("Argon2 error: {0}")]
    //Argon2Error(#[from] argon2::Error),
    #[error("Header conversion error: {0}")]
    HeaderToStrError(#[from] ToStrError),
}
