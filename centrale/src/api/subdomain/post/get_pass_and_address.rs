use crate::{db::get_db::get_centrale_db, error::CentraleError};
use actix_web::web::Data;
use r2d2::Pool;
use r2d2_sqlite::{SqliteConnectionManager, rusqlite::params};

pub struct SubdomainData {
    pub password: String,
    pub address: String,
    pub destination_bearer: String,
    pub name: String,
    pub serve_front: bool,
}
///
pub fn get_subdomain_data(
    pool: &Data<Pool<SqliteConnectionManager>>,
    subdomain: &String,
) -> Result<SubdomainData, CentraleError> {
    if subdomain == "app" {
        // ALLOW VISITS TO app FOR ALL AUTHENTICATED USERS
        return Ok(SubdomainData {
            password: "pass".to_string(),
            address: "app".to_string(),
            destination_bearer: "bearer".to_string(),
            name: "app".to_string(),
            serve_front: false,
        });
    }

    let db = get_centrale_db(pool.get_ref())?;
    let mut stmt = db.prepare(
        "SELECT password, address, destination_bearer, name, serve_front FROM subdomain WHERE subdomain = ?1",
    )?;

    let data = stmt.query_row(params![subdomain], |row| {
        Ok(SubdomainData {
            password: row.get(0)?,
            address: row.get(1)?,
            destination_bearer: row.get(2)?,
            name: row.get(3)?,
            serve_front: row.get(4)?,
        })
    })?;

    Ok(data)
}
