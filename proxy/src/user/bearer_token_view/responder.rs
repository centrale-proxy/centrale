use crate::user::bearer_token_view::process::process_view_bearer_tokens;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use dir_and_db_pool::db::DbBool;
use log::error;

pub async fn view_bearer_tokens(
    pool: web::Data<DbBool>,
    req: HttpRequest,
    user_id: web::Path<i64>,
) -> impl Responder {
    match process_view_bearer_tokens(pool, req, user_id) {
        Ok(result) => result,
        Err(err) => {
            error!("Bearer token error: {}", err);
            HttpResponse::UnprocessableEntity()
                .json(serde_json::json!({ "error": "Cannot generate bearer token" }))
        }
    }
}
