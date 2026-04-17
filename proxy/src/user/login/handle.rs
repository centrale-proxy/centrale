use crate::user::login::process::{LoginUser, process_login};
use actix_web::{HttpResponse, Responder, web};
use dir_and_db_pool::db::DbPool;
use log::error;
/// Login user user responder
pub async fn handle_login(pool: web::Data<DbPool>, json: web::Json<LoginUser>) -> impl Responder {
    match process_login(pool, json) {
        Ok(result) => result,
        Err(err) => {
            error!("Login error: {}", err);
            HttpResponse::UnprocessableEntity()
                .json(serde_json::json!({ "error": "Cannot log in" }))
        }
    }
}
