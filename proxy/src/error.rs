use actix_http::{StatusCode, header::ToStrError};
use actix_web::ResponseError;
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

    #[error("Invalid method")]
    InvalidMethod,

    #[error("URL parse error: {0}")]
    UrlParseError(#[from] ParseError),

    #[error("No token or cookie present")]
    NoTokenOrCookiePresent,

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

    #[error("Password is to long. Max allowed length: 200 chars")]
    PasswordIsTooLong,

    #[error("No cookie found")]
    NoCookie,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Wrong user for token")]
    WrongUser,

    #[error("Wrong cookie")]
    InvalidCookie,

    #[error("Wrong token")]
    InvalidToken,
}

impl ResponseError for CentraleError {
    fn status_code(&self) -> StatusCode {
        match self {
            CentraleError::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CentraleError::StringError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code())
            .json(serde_json::json!({"error": self.to_string()}))
    }
}
