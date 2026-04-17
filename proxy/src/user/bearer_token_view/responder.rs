use crate::{
    server::auth::CentraleUser, user::bearer_token_view::process::process_view_bearer_tokens,
};
use actix_web::{HttpResponse, Responder, web};
use dir_and_db_pool::db::DbBool;
use log::error;

pub async fn view_bearer_tokens(
    pool: web::Data<DbBool>,
    user_id: web::Path<i64>,
    user: CentraleUser,
) -> impl Responder {
    match process_view_bearer_tokens(pool, user_id, user) {
        Ok(result) => result,
        Err(err) => {
            error!("Bearer token error: {}", err);
            HttpResponse::UnprocessableEntity()
                .json(serde_json::json!({ "error": "Cannot generate bearer token" }))
        }
    }
}
