use crate::error::CentraleError;
use actix_web::web::Data;
use r2d2::Pool;
use r2d2_sqlite::{SqliteConnectionManager, rusqlite::params};

pub fn get_subdomain_user_role(
    pool: &Data<Pool<SqliteConnectionManager>>,
    subdomain: &String,
    user_id: i64,
) -> Result<String, CentraleError> {
    let db = pool.get()?;
    let mut stmt =
        db.prepare(&"SELECT role FROM subdomain_user WHERE subdomain = ?1 AND user_id = ?2")?;
    let role: String = stmt.query_row(params![subdomain, user_id], |row| row.get(0))?;
    Ok(role)
}
