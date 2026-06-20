use crate::{db::get_db::get_centrale_db, error::CentraleError};
use actix_web::web::Data;
use r2d2::Pool;
use r2d2_sqlite::{SqliteConnectionManager, rusqlite::params};

pub fn get_subdomain_user_role(
    pool: &Data<Pool<SqliteConnectionManager>>,
    subdomain: &String,
    user_id: i64,
) -> Result<String, CentraleError> {
    if subdomain == "app" {
        // ALLOW VISITS TO app FOR ALL AUTHENTICATED USERS
        return Ok("user".to_string());
    }
    let db = get_centrale_db(pool.get_ref())?;
    let mut stmt =
        db.prepare(&"SELECT role FROM subdomain_user WHERE subdomain = ?1 AND user_id = ?2")?;
    let role: String = stmt.query_row(params![subdomain, user_id], |row| row.get(0))?;
    Ok(role)
}
