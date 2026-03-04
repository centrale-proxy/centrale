use r2d2::PooledConnection;
use r2d2_sqlite::{SqliteConnectionManager, rusqlite::params};

use crate::{error::CentraleError, user::hash::hash_password};

pub fn add_user(
    db: &PooledConnection<SqliteConnectionManager>,
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
