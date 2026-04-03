use crate::{hello::process::process_hello, pool::DbPoolRegistry};
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use std::sync::{Arc, RwLock};

pub async fn respond_hello(
    pool: web::Data<Arc<RwLock<DbPoolRegistry>>>,
    req: HttpRequest,
) -> impl Responder {
    match process_hello(pool, req) {
        Ok(result) => result,
        Err(err) => HttpResponse::UnprocessableEntity()
            .json(serde_json::json!({ "error": err.to_string() })),
    }
}
