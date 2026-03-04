use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use log::error;
use rand::rngs::OsRng;

use crate::error::CentraleError;

pub fn hash_password(password: &str) -> Result<(String, String), CentraleError> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let hashed_password_result = argon2.hash_password(password.as_bytes(), &salt);

    match hashed_password_result {
        Ok(hashed_pass) => {
            let salt_string = salt.as_str().to_string();
            let res = (hashed_pass.to_string(), salt_string);
            return Ok(res);
        }
        Err(err) => {
            error!("argon error: {}", err);
            Err(CentraleError::UnableToHash)
        }
    }
}
