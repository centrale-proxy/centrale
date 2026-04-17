use crate::{db::get_db::get_centrale_db, error::CentraleError};
use actix_web::web::Data;
use r2d2::Pool;
use r2d2_sqlite::{SqliteConnectionManager, rusqlite::params};

pub fn get_subdomain_pass(
    pool: &Data<Pool<SqliteConnectionManager>>,
    subdomain: &String,
) -> Result<String, CentraleError> {
    if subdomain == "app" {
        // ALLOW VISITS TO app FOR ALL AUTHENTICATED USERS
        return Ok("pass".to_string());
    }
    let db = get_centrale_db(pool.get_ref())?;
    let mut stmt = db.prepare(&"SELECT password FROM subdomain WHERE subdomain = ?1")?;
    let pass: String = stmt.query_row(params![subdomain], |row| row.get(0))?;
    Ok(pass)
}
