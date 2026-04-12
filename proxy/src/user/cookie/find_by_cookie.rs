use crate::{db::get_db::get_centrale_db, error::CentraleError};
use actix_web::web::Data;
use r2d2::Pool;
use r2d2_sqlite::{SqliteConnectionManager, rusqlite::params};

pub fn find_user_by_cookie(
    pool: &Data<Pool<SqliteConnectionManager>>,
    cookie: &String,
) -> Result<i64, CentraleError> {
    let db = get_centrale_db(pool.get_ref())?;
    let mut stmt = db.prepare(&"SELECT user_id FROM cookie WHERE cookie = ?1")?;
    let user_id: i64 = stmt.query_row(params![cookie], |row| row.get(0))?;
    Ok(user_id)
}
