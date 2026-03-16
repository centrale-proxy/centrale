use actix_web::{HttpRequest, HttpResponse, Responder, web};
use dir_and_db_pool::db::DbBool;
use log::error;

use crate::user::get::process::get_user_process;
//

pub async fn get_user(pool: web::Data<DbBool>, req: HttpRequest) -> impl Responder {
    match get_user_process(pool, req) {
        Ok(result) => result,
        Err(err) => {
            error!("Get user error: {}", err);
            HttpResponse::UnprocessableEntity()
                .json(serde_json::json!({ "error": "Cannot get user" }))
        }
    }
}
