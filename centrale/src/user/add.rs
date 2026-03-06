use crate::{error::CentraleError, user::hash::hash_password};
use dir_and_db_pool::db::DbConnection;
use r2d2_sqlite::rusqlite::params;

pub fn add_user(
    db: &DbConnection,
    username: &String,
    password: &String,
) -> Result<i64, CentraleError> {
    let mut stmt = db.prepare(&"SELECT COUNT(*) FROM user WHERE username = ?1")?;
    let count: i64 = stmt.query_row(params![username], |row| row.get(0))?;

    if count > 0 {
        // USERS(s) EXIST. CANNOT HAVE MORE
        return Err(CentraleError::SuchUserExists);
    } else {
        let (hash, salt) = hash_password(&password)?;
        // INSERT TO DB
        db.execute(
            "INSERT INTO user (username, password, salt) VALUES (?1, ?2, ?3)",
            params![username, hash, salt],
        )?;
        let last_id = db.last_insert_rowid();
        Ok(last_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init::init_db;
    use r2d2::Pool;
    use r2d2_sqlite::SqliteConnectionManager;

    #[test]
    fn add_user_ok() {
        let manager = SqliteConnectionManager::memory();
        let pool = Pool::new(manager).expect("Failed to create pool.");
        init_db(&pool).unwrap();
        let db = pool.get().expect("Couldn't get db connection from pool");
        let id = add_user(&db, &"username".to_string(), &"password".to_string()).unwrap();
        assert!(id == 1)
    }

    #[test]
    fn add_same_id_errors() {
        let manager = SqliteConnectionManager::memory();
        let pool = Pool::new(manager).expect("Failed to create pool.");
        init_db(&pool).unwrap();
        let db = pool.get().expect("Couldn't get db connection from pool");
        let _id = add_user(&db, &"username".to_string(), &"password".to_string()).unwrap();
        let second = add_user(&db, &"username".to_string(), &"password".to_string());
        assert!(second.is_err());
    }
}
