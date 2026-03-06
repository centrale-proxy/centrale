use crate::user::register::{RegisterUser, handle_register};
use actix_web::{HttpResponse, Responder, web};
use dir_and_db_pool::db::DbBool;
use log::error;

pub async fn post_user(pool: web::Data<DbBool>, json: web::Json<RegisterUser>) -> impl Responder {
    match handle_register(pool, json) {
        Ok(result) => HttpResponse::Ok().json(serde_json::json!({ "id": result.to_string() })),
        Err(err) => {
            error!("Add user error: {}", err);
            HttpResponse::Unauthorized().json(serde_json::json!({ "error": err.to_string() }))
        }
    }
}
