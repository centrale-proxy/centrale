use crate::error::CentraleError;
use dir_and_db_pool::db::DbConnection;
use r2d2_sqlite::rusqlite::params;

/// Add user to db
pub fn add_user_to_db(
    db: &DbConnection,
    username: &String,
    hash: &String,
    salt: &str,
) -> Result<i64, CentraleError> {
    db.execute(
        "INSERT INTO user (username, password, salt) VALUES (?1, ?2, ?3)",
        params![username, hash, salt],
    )?;

    let last_id = db.last_insert_rowid();
    Ok(last_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db::init::init_db, user::post::hash_and_salt::hash_and_salt};
    use r2d2::Pool;
    use r2d2_sqlite::SqliteConnectionManager;

    #[test]
    fn add_user_ok() {
        let manager = SqliteConnectionManager::memory();
        let pool = Pool::new(manager).expect("Failed to create pool.");
        init_db(&pool).unwrap();
        let db = pool.get().expect("Couldn't get db connection from pool");

        let (hash, salt) = hash_and_salt(&"password".to_string()).unwrap();
        let user_id = add_user_to_db(&db, &"username".to_string(), &hash, salt.as_str()).unwrap();
        assert!(user_id == 1)
    }

    #[test]
    fn add_same_id_errors() {
        let manager = SqliteConnectionManager::memory();
        let pool = Pool::new(manager).expect("Failed to create pool.");
        init_db(&pool).unwrap();
        let db = pool.get().expect("Couldn't get db connection from pool");
        let (hash, salt) = hash_and_salt(&"password".to_string()).unwrap();
        let _user_id = add_user_to_db(&db, &"username".to_string(), &hash, salt.as_str()).unwrap();
        let second = add_user_to_db(&db, &"username".to_string(), &hash, salt.as_str());
        assert!(second.is_err());
    }

    #[test]
    fn add_0_byte_user_errors() {
        let manager = SqliteConnectionManager::memory();
        let pool = Pool::new(manager).expect("Failed to create pool.");
        init_db(&pool).unwrap();
        let db = pool.get().expect("Couldn't get db connection from pool");

        let (hash, salt) = hash_and_salt(&"password".to_string()).unwrap();

        let user_id = add_user_to_db(&db, &"\0".to_string(), &hash, salt.as_str());

        assert!(user_id.is_err());
    }

    #[test]
    fn add_101_chars_user_errors() {
        let manager = SqliteConnectionManager::memory();
        let pool = Pool::new(manager).expect("Failed to create pool.");
        init_db(&pool).unwrap();
        let db = pool.get().expect("Couldn't get db connection from pool");

        let (hash, salt) = hash_and_salt(&"password".to_string()).unwrap();

        let user_id = add_user_to_db(&db, &"01234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567891".to_string(), &hash, salt.as_str());

        assert!(user_id.is_err());
    }

    #[test]
    fn pass_with_501_chars_produces_normal_size_hash_and_does_not_error() {
        let manager = SqliteConnectionManager::memory();
        let pool = Pool::new(manager).expect("Failed to create pool.");
        init_db(&pool).unwrap();
        let db = pool.get().expect("Couldn't get db connection from pool");

        let (hash, salt) = hash_and_salt(&"01234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567891012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678910123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789101234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567891012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678191".to_string()).unwrap();

        assert!(hash.len() < 100);

        let user_id = add_user_to_db(&db, &"user".to_string(), &hash, salt.as_str());

        assert!(user_id.is_ok());
    }
}
