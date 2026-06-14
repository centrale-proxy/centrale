use crate::{db::get_db::get_centrale_db, error::CentraleError};
use actix_web::web::Data;
use r2d2::Pool;
use r2d2_sqlite::{SqliteConnectionManager, rusqlite::params};

pub fn find_user_salt(
    pool: &Data<Pool<SqliteConnectionManager>>,
    username: &String,
) -> Result<String, CentraleError> {
    let db = get_centrale_db(pool.get_ref())?;
    let mut stmt = db.prepare(&"SELECT salt FROM user WHERE username = ?1")?;
    let salt: String = stmt.query_row(params![username], |row| row.get(0))?;
    Ok(salt)
}
