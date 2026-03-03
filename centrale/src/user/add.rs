use crate::user::register::{RegisterUser, handle_register};
use actix_web::{HttpResponse, Responder, web};
use log::error;

pub async fn add_user(json: web::Json<RegisterUser>) -> impl Responder {
    match handle_register(json) {
        Ok(result) => HttpResponse::Ok().body(result),
        Err(err) => {
            error!("Add user error: {}", err);
            HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Not authenticated" }))
        }
    }
}
