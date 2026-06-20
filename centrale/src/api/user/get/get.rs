use crate::server::auth::CentraleUser;
use actix_web::{HttpResponse, Responder};

pub async fn get_user(user: CentraleUser) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "user_id": user.user_id.to_string() }))
}
