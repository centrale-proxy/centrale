use crate::error::CentraleError;
use actix_web::web::Data;
use config::CentraleConfig;
use dir_and_db_pool::db::get_encrypted_connection::get_encrypted_connection;
use r2d2::Pool;
use r2d2_sqlite::{SqliteConnectionManager, rusqlite::params};

pub fn find_user_salt(
    pool: &Data<Pool<SqliteConnectionManager>>,
    username: &String,
) -> Result<String, CentraleError> {
    let db = get_encrypted_connection(pool.get_ref(), CentraleConfig::MASTER_PASSWORD)?;
    let mut stmt = db.prepare(&"SELECT salt FROM user WHERE username = ?1")?;
    let salt: String = stmt.query_row(params![username], |row| row.get(0))?;
    Ok(salt)
}
