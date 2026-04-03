use crate::{db::get_db::get_encrypted_connection, error::CentraleError};
use actix_web::web::Data;
use r2d2::Pool;
use r2d2_sqlite::{SqliteConnectionManager, rusqlite::params};

pub fn find_user_by_token(
    pool: &Data<Pool<SqliteConnectionManager>>,
    bearer: &String,
) -> Result<i64, CentraleError> {
    let db = get_encrypted_connection(pool.get_ref())?;
    let mut stmt = db.prepare(&"SELECT user_id FROM bearer WHERE bearer = ?1")?;
    let user_id: i64 = stmt.query_row(params![bearer], |row| row.get(0))?;
    Ok(user_id)
}
