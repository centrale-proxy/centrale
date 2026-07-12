use crate::{
    api::subdomain_user::post::handle::SubdomainUserAndRole, error::CentraleError,
    server::auth::CentraleUser,
};
use actix_web::{HttpResponse, web};
use dir_and_db_pool::db::DbPool;
use r2d2_sqlite::rusqlite::params;

pub fn logic_subdomain_add_user(
    pool: web::Data<DbPool>,
    json: web::Json<SubdomainUserAndRole>,
    user: CentraleUser,
) -> Result<HttpResponse, CentraleError> {
    let db = pool.get()?;
    let body = json.into_inner();

    // Only an admin of that subdomain may add/update users on it
    if user.subdomain != body.subdomain || user.role != "admin" {
        return Ok(HttpResponse::Forbidden().body("not an admin of this subdomain"));
    }
    // Upsert: update the role if the mapping exists, otherwise insert it
    let updated = db.execute(
        "UPDATE subdomain_user SET role = ?1 WHERE subdomain = ?2 AND user_id = ?3",
        params![body.role, body.subdomain, body.user_id],
    )?;

    if updated == 0 {
        db.execute(
            "INSERT INTO subdomain_user (subdomain, user_id, role) VALUES (?1, ?2, ?3)",
            params![body.subdomain, body.user_id, body.role],
        )?;
    }

    Ok(HttpResponse::Ok().finish())
}
