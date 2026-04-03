use crate::register::process::process_register;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use dir_and_db_pool::db::DbBool;

pub async fn respond_register(pool: web::Data<DbBool>, req: HttpRequest) -> impl Responder {
    match process_register(pool, req) {
        Ok(result) => result,
        Err(err) => HttpResponse::UnprocessableEntity()
            .json(serde_json::json!({ "error": err.to_string() })),
    }
}
