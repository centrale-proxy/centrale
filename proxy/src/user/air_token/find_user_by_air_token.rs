use crate::{db::get_db::get_centrale_db, error::CentraleError};
use actix_web::web::Data;
use chrono::Utc;
use r2d2::Pool;
use r2d2_sqlite::{SqliteConnectionManager, rusqlite::params};

pub fn find_user_by_air_token(
    pool: &Data<Pool<SqliteConnectionManager>>,
    air_token: &String,
) -> Result<i64, CentraleError> {
    let db = get_centrale_db(pool.get_ref())?;

    let mut stmt = db.prepare("SELECT user_id, timeout FROM air_token WHERE air_token = ?1")?;

    let (user_id, timeout): (i64, i64) =
        stmt.query_row(params![air_token], |row| Ok((row.get(0)?, row.get(1)?)))?;

    let now = Utc::now().timestamp();

    if now > timeout {
        // Token expired — delete it and return error
        db.execute(
            "DELETE FROM air_token WHERE air_token = ?1",
            params![air_token],
        )?;
        return Err(CentraleError::AirTokenExpired);
    }

    // Token valid but single-use — delete after reading
    db.execute(
        "DELETE FROM air_token WHERE air_token = ?1",
        params![air_token],
    )?;

    Ok(user_id)
}
