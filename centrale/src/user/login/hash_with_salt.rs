use crate::{error::CentraleError, user::hash::hash_password};
use argon2::password_hash::SaltString;

pub fn hash_with_salt(password: &String, salt_string: &str) -> Result<String, CentraleError> {
    let salt = SaltString::from_b64(salt_string)?;
    let hash = hash_password(&password, &salt)?;
    Ok(hash)
}
