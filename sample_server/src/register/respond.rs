use crate::{pool::DbPoolRegistry, register::process::process_register};
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use std::sync::{Arc, RwLock};

pub async fn respond_register(
    registry: web::Data<Arc<RwLock<DbPoolRegistry>>>,
    req: HttpRequest,
) -> impl Responder {
    match process_register(registry, req) {
        Ok(result) => result,
        Err(err) => HttpResponse::UnprocessableEntity()
            .json(serde_json::json!({ "error": err.to_string() })),
    }
}
