use crate::{db::get_db::get_centrale_db, error::CentraleError};
use actix_web::web::Data;
use r2d2::Pool;
use r2d2_sqlite::{SqliteConnectionManager, rusqlite::params};

pub fn find_user_by_hash_and_username(
    pool: &Data<Pool<SqliteConnectionManager>>,
    hash: &String,
    username: &str,
) -> Result<i64, CentraleError> {
    let db = get_centrale_db(pool.get_ref())?;
    let mut stmt = db.prepare(&"SELECT id FROM user WHERE username = ?1 AND password = ?2")?;
    let user_id: i64 = stmt.query_row(params![username, hash], |row| row.get(0))?;
    Ok(user_id)
}
