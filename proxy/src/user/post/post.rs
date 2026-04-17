use actix_web::{HttpResponse, Responder, web};
use dir_and_db_pool::db::DbPool;
use log::error;

use crate::user::post::process::{RegisterUser, handle_register};
/// Post user responsder
pub async fn post_user(pool: web::Data<DbPool>, json: web::Json<RegisterUser>) -> impl Responder {
    match handle_register(pool, json) {
        Ok(result) => result,
        Err(err) => {
            error!("Add user error: {}", err);
            HttpResponse::UnprocessableEntity()
                .json(serde_json::json!({ "error": "Cannot add user" }))
        }
    }
}
