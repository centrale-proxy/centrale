use crate::{db::get_db::get_centrale_db, error::CentraleError};
use actix_web::web::Data;
use r2d2::Pool;
use r2d2_sqlite::{SqliteConnectionManager, rusqlite::params};

pub fn find_bearer_tokens(
    pool: &Data<Pool<SqliteConnectionManager>>,
    user_id: i64,
) -> Result<Vec<String>, CentraleError> {
    let db = get_centrale_db(pool.get_ref())?;

    let mut stmt = db.prepare("SELECT bearer FROM bearer WHERE user_id = ?1")?;

    let rows = stmt.query_map(params![user_id], |row| row.get::<_, String>(0))?;

    let mut bearers = Vec::new();

    for bearer in rows {
        bearers.push(bearer?);
    }

    Ok(bearers)
}
