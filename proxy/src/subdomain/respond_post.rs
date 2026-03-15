use crate::subdomain::handle_post::{RegisterSubdomain, handle_post};
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use dir_and_db_pool::db::DbBool;
use log::error;

pub async fn respond_subdomain(
    pool: web::Data<DbBool>,
    json: web::Json<RegisterSubdomain>,
    req: HttpRequest,
) -> impl Responder {
    match handle_post(pool, json, req) {
        Ok(result) => result,
        Err(err) => {
            error!("Add subdomain error: {}", err);
            HttpResponse::UnprocessableEntity()
                .json(serde_json::json!({ "error": "Cannot post subdomain" }))
        }
    }
}
