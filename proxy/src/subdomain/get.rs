use crate::error::CentraleError;
use actix_web::web::Data;
use config::CentraleConfig;
use dir_and_db_pool::db::get_encrypted_connection::get_encrypted_connection;
use r2d2::Pool;
use r2d2_sqlite::{SqliteConnectionManager, rusqlite::params};

pub fn get_subdomain_pass(
    pool: &Data<Pool<SqliteConnectionManager>>,
    subdomain: &String,
) -> Result<String, CentraleError> {
    let db = get_encrypted_connection(pool.get_ref(), CentraleConfig::MASTER_PASSWORD)?;
    let mut stmt = db.prepare(&"SELECT password FROM subdomain WHERE subdomain = ?1")?;
    let pass: String = stmt.query_row(params![subdomain], |row| row.get(0))?;
    Ok(pass)
}
