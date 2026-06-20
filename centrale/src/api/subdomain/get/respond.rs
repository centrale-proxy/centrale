use crate::{api::subdomain::get::process::process_get_subdomain, server::auth::CentraleUser};
use actix_web::{HttpResponse, Responder, web};
use dir_and_db_pool::db::DbPool;
use log::error;

pub async fn respond_get_subdomain(pool: web::Data<DbPool>, user: CentraleUser) -> impl Responder {
    match process_get_subdomain(pool, user) {
        Ok(result) => result,
        Err(err) => {
            error!("/api/subdomain error: {}", err);
            HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Not Ok" }))
        }
    }
}
