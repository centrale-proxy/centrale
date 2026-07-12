use crate::{db::get_db::get_centrale_db, error::CentraleError, server::auth::CentraleUser};
use actix_web::{HttpResponse, web};
use dir_and_db_pool::db::DbPool;
use r2d2_sqlite::rusqlite::params;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetSubdomainUsers {
    pub subdomain: String,
    pub subdomain_name: Option<String>,
    pub role: String,
    pub user_id: i64,
    pub username: String,
}

/// Get subdomain users
pub fn process_get_subdomain_user(
    pool: web::Data<DbPool>,
    user: CentraleUser,
) -> Result<HttpResponse, CentraleError> {
    if user.role == "admin" {
        let db = get_centrale_db(pool.get_ref())?;
        let mut stmt = db.prepare(
            "SELECT su.subdomain, s.name, su.role, su.user_id, u.username
             FROM subdomain_user su
             JOIN subdomain s ON s.subdomain = su.subdomain
             JOIN user u ON u.id = su.user_id
             WHERE su.subdomain = ?1",
        )?;
        let data: Vec<GetSubdomainUsers> = stmt
            .query_map(params![user.subdomain], |row| {
                Ok(GetSubdomainUsers {
                    subdomain: row.get(0)?,
                    subdomain_name: row.get(1)?,
                    role: row.get(2)?,
                    user_id: row.get(3)?,
                    username: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(HttpResponse::Ok().json(data))
    } else {
        Ok(HttpResponse::Unauthorized().json(serde_json::json!({"error": "Unauthorized"})))
    }
}
