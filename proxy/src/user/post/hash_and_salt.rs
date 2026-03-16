use crate::{error::CentraleError, user::hash::hash_password};
use argon2::password_hash::SaltString;
use rand::rngs::OsRng;

/// Create salt, hash password and save to db
pub fn hash_and_salt(password: &String) -> Result<(String, String), CentraleError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = hash_password(&password, &salt)?;
    let salt_str = salt.as_str().to_string();
    Ok((hash, salt_str))
}
