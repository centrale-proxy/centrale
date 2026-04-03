use actix_web::{HttpRequest, HttpResponse, Responder, web};
use dir_and_db_pool::db::DbBool;
use log::error;

use crate::user::air_token::process::process_air_token;
//

pub async fn generate_air_token(pool: web::Data<DbBool>, req: HttpRequest) -> impl Responder {
    match process_air_token(pool, req) {
        Ok(result) => result,
        Err(err) => {
            error!("Air token error: {}", err);
            HttpResponse::UnprocessableEntity()
                .json(serde_json::json!({ "error": "Cannot generate air token" }))
        }
    }
}
