use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use log::error;

use crate::error::CentraleError;

pub fn hash_password(password: &str, salt: &SaltString) -> Result<String, CentraleError> {
    let argon2 = Argon2::default();
    let hashed_password_result = argon2.hash_password(password.as_bytes(), salt);

    match hashed_password_result {
        Ok(hashed_pass) => {
            return Ok(hashed_pass.to_string());
        }
        Err(err) => {
            error!("argon error: {}", err);
            Err(CentraleError::UnableToHash)
        }
    }
}
