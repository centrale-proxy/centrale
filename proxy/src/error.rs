use actix_http::header::ToStrError;
use dir_and_db_pool::error::DirsqlError;
use r2d2::Error as R2d2Error;
use r2d2_sqlite::rusqlite;
use std::error::Error as StdError;
use thiserror::Error;
use url::ParseError;

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

    #[error("Missing host")]
    MissingHost,

    #[error("Invalid domain")]
    InvalidDomain,

    #[error("Invalid subdomain")]
    InvalidSubdomain,

    #[error("URL parse error: {0}")]
    UrlParseError(#[from] ParseError),

    #[error("No token or cookie present")]
    NoTokenOrCookiePresent,

    #[error("Air token expired")]
    AirTokenExpired,

    #[error("No air token")]
    NoAirToken,

    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("Database pool error: {0}")]
    PoolError(#[from] R2d2Error),

    #[error("Such subdomain exists")]
    SuchSubdomainExists,

    #[error("Unable to hash")]
    UnableToHash,
    //#[error("Argon2 error: {0}")]
    //Argon2Error(#[from] argon2::Error),
    #[error("Header conversion error: {0}")]
    HeaderToStrError(#[from] ToStrError),

    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("invalid UTF-8: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Proxy request error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("DirsqlError: {0}")]
    DirsqlError(#[from] DirsqlError),

    #[error("Password hash error: {0}")]
    PasswordHash(#[from] argon2::password_hash::Error),
    /*
    #[error("Missing CENTRALE_MASTER_PASSWORD in environment variables")]
    MissingMasterPassword,

    #[error("Missing CENTRALE_MASTER_BEARER_TOKEN in environment variables")]
    MissingMasterBearerToken,
     */
}
