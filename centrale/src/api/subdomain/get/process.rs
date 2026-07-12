use crate::{db::get_db::get_centrale_db, error::CentraleError, server::auth::CentraleUser};
use actix_web::{HttpResponse, web};
use dir_and_db_pool::db::DbPool;
use r2d2_sqlite::rusqlite::params;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SubdomainNameAndRole {
    pub subdomain: String,
    pub name: Option<String>,
    pub role: String,
    pub user_id: i64,
}

/// Get user subdomains
pub fn process_get_subdomain(
    pool: web::Data<DbPool>,
    user: CentraleUser,
) -> Result<HttpResponse, CentraleError> {
    let db = get_centrale_db(pool.get_ref())?;
    let mut stmt = db.prepare(
        "SELECT su.subdomain, s.name, su.role, su.user_id
         FROM subdomain_user su
         JOIN subdomain s ON s.subdomain = su.subdomain
         WHERE su.user_id = ?1",
    )?;
    let data: Vec<SubdomainNameAndRole> = stmt
        .query_map(params![user.user_id], |row| {
            Ok(SubdomainNameAndRole {
                subdomain: row.get(0)?,
                name: row.get(1)?,
                role: row.get(2)?,
                user_id: row.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(HttpResponse::Ok().json(data))
}
