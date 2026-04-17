use crate::{
    server::auth::CentraleUser, user::bearer_token::process::process_generate_bearer_token,
};
use actix_web::{HttpResponse, Responder, web};
use dir_and_db_pool::db::DbBool;
use log::error;

pub async fn generate_bearer_token(
    pool: web::Data<DbBool>,
    user_id: web::Path<i64>,
    user: CentraleUser,
) -> impl Responder {
    match process_generate_bearer_token(pool, user_id, user) {
        Ok(result) => result,
        Err(err) => {
            error!("Bearer token error: {}", err);
            HttpResponse::UnprocessableEntity()
                .json(serde_json::json!({ "error": "Cannot generate bearer token" }))
        }
    }
}
