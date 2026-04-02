use crate::hello::process::process_hello;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use dir_and_db_pool::db::DbBool;

pub async fn respond_hello(pool: web::Data<DbBool>, req: HttpRequest) -> impl Responder {
    match process_hello(pool, req) {
        Ok(result) => result,
        Err(err) => HttpResponse::UnprocessableEntity()
            .json(serde_json::json!({ "error": err.to_string() })),
    }
}
