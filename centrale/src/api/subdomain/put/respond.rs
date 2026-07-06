use crate::{
    api::subdomain::{get::process::SubdomainAndName, put::process::process_put_subdomain},
    server::auth::CentraleUser,
};
use actix_web::{HttpResponse, Responder, web};
use dir_and_db_pool::db::DbPool;
use log::error;

pub async fn put_subdomain(
    pool: web::Data<DbPool>,
    user: CentraleUser,
    url_id: web::Path<String>,
    body: web::Json<SubdomainAndName>,
) -> impl Responder {
    match process_put_subdomain(pool, url_id, body, user) {
        Ok(result) => result,
        Err(err) => {
            error!("/api/subdomain error: {}", err);
            HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Not Ok" }))
        }
    }
}
