use crate::{
    api::subdomain_user::get::process::process_get_subdomain_user, server::auth::CentraleUser,
};
use actix_web::{HttpResponse, Responder, web};
use dir_and_db_pool::db::DbPool;
use log::error;

pub async fn respond_get_subdomain_user(
    pool: web::Data<DbPool>,
    user: CentraleUser,
) -> impl Responder {
    match process_get_subdomain_user(pool, user) {
        Ok(result) => result,
        Err(err) => {
            error!("/api/subdomain_user error: {}", err);
            HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Not Ok" }))
        }
    }
}
