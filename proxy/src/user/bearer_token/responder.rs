use actix_web::{HttpRequest, HttpResponse, Responder, web};
use dir_and_db_pool::db::DbBool;
use log::error;

use crate::user::bearer_token::process::process_generate_bearer_token;
//

pub async fn generate_bearer_token(
    pool: web::Data<DbBool>,
    req: HttpRequest,
    user_id: web::Path<i64>,
) -> impl Responder {
    match process_generate_bearer_token(pool, req, user_id) {
        Ok(result) => result,
        Err(err) => {
            error!("Bearer token error: {}", err);
            HttpResponse::UnprocessableEntity()
                .json(serde_json::json!({ "error": "Cannot generate bearer token" }))
        }
    }
}
