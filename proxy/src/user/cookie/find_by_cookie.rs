use crate::{db::get_db::get_centrale_db, error::CentraleError};
use actix_web::web::Data;
use chrono::Utc;
use r2d2::Pool;
use r2d2_sqlite::{SqliteConnectionManager, rusqlite::params};

pub fn find_user_by_cookie(
    pool: &Data<Pool<SqliteConnectionManager>>,
    cookie: &String,
) -> Result<i64, CentraleError> {
    let db = get_centrale_db(pool.get_ref())?;

    let mut stmt = db.prepare("SELECT user_id, timeout FROM cookie WHERE cookie = ?1")?;

    let (user_id, timeout): (i64, i64) =
        stmt.query_row(params![cookie], |row| Ok((row.get(0)?, row.get(1)?)))?;

    let current_unix_epoch = Utc::now().timestamp();

    if timeout < current_unix_epoch {
        // Delete expired cookie
        db.execute("DELETE FROM cookie WHERE cookie = ?1", params![cookie])?;

        return Err(CentraleError::Unauthorized); // or a custom error
    }

    Ok(user_id)
}
