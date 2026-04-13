use crate::{error::CentraleError, user::hash::hash_password};
use argon2::password_hash::SaltString;
use rand::rngs::OsRng;

/// Create salt, hash password and save to db
pub fn hash_and_salt(password: &String) -> Result<(String, String), CentraleError> {
    if password.len() > 200 {
        return Err(CentraleError::PasswordIsTooLong);
    }
    let salt = SaltString::generate(&mut OsRng);
    let hash = hash_password(&password, &salt)?;
    let salt_str = salt.as_str().to_string();
    Ok((hash, salt_str))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_cannot_be_over_200_chars() {
        let aaa = hash_and_salt(&"012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678910123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789".to_string());
        assert!(aaa.is_err());
    }
}
