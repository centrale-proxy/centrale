use crate::{
    api::subdomain::get::process::SubdomainAndName, db::get_db::get_centrale_db,
    error::CentraleError, server::auth::CentraleUser,
};
use actix_web::{HttpResponse, web};
use dir_and_db_pool::db::DbPool;
use r2d2_sqlite::rusqlite::params;

/// Put one subdomain — updates only the name (admin of that subdomain only)
pub fn process_put_subdomain(
    pool: web::Data<DbPool>,
    subdomain_id: web::Path<String>,
    body: web::Json<SubdomainAndName>,
    user: CentraleUser,
) -> Result<HttpResponse, CentraleError> {
    let old_subdomain = subdomain_id.into_inner();

    // Only an admin of this exact subdomain may change it
    if user.subdomain != old_subdomain || user.role != "admin" {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "forbidden",
            "message": "Only an admin of this subdomain can modify it",
        })));
    }

    let new = body.into_inner();
    let db = get_centrale_db(pool.get_ref())?;

    let rows = db.execute(
        "UPDATE subdomain SET name = ?1 WHERE subdomain = ?2",
        params![new.name, old_subdomain],
    )?;

    if rows == 0 {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "subdomain_not_found",
            "message": format!("No subdomain with id '{}' exists", old_subdomain),
        })));
    }

    Ok(HttpResponse::Ok().json(SubdomainAndName {
        name: new.name,
        subdomain: old_subdomain,
    }))
}
