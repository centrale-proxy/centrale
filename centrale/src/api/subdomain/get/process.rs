use crate::{db::get_db::get_centrale_db, error::CentraleError, server::auth::CentraleUser};
use actix_web::{HttpResponse, web};
use dir_and_db_pool::db::DbPool;
use r2d2_sqlite::rusqlite::params;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SubdomainAndRole {
    pub subdomain: String,
    pub role: String,
}

/// Get user subdomains
pub fn process_get_subdomain(
    pool: web::Data<DbPool>,
    user: CentraleUser,
) -> Result<HttpResponse, CentraleError> {
    let db = get_centrale_db(pool.get_ref())?;
    let mut stmt = db.prepare("SELECT subdomain, role FROM subdomain_user WHERE user_id = ?1")?;
    let data: Vec<SubdomainAndRole> = stmt
        .query_map(params![user.user_id], |row| {
            Ok(SubdomainAndRole {
                subdomain: row.get(0)?,
                role: row.get(1)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(HttpResponse::Ok().json(data))
}
