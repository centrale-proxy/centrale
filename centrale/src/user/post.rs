use crate::user::register::{RegisterUser, handle_register};
use actix_web::{HttpResponse, Responder, web};
use log::error;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub async fn post_user(
    pool: web::Data<Pool<SqliteConnectionManager>>,
    json: web::Json<RegisterUser>,
) -> impl Responder {
    match handle_register(pool, json) {
        Ok(result) => HttpResponse::Ok().body(result),
        Err(err) => {
            error!("Add user error: {}", err);
            HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Not authenticated" }))
        }
    }
}
