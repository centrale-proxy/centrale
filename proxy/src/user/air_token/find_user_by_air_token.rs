use crate::error::CentraleError;
use actix_web::web::Data;
use config::CentraleConfig;
use dir_and_db_pool::db::get_encrypted_connection::get_encrypted_connection;
use r2d2::Pool;
use r2d2_sqlite::{SqliteConnectionManager, rusqlite::params};

pub fn find_user_by_air_token(
    pool: &Data<Pool<SqliteConnectionManager>>,
    air_token: &String,
) -> Result<i64, CentraleError> {
    let db = get_encrypted_connection(pool.get_ref(), CentraleConfig::MASTER_PASSWORD)?;
    let mut stmt = db.prepare(&"SELECT user_id FROM air_token WHERE air_token = ?1")?;
    let user_id: i64 = stmt.query_row(params![air_token], |row| row.get(0))?;
    Ok(user_id)
}
