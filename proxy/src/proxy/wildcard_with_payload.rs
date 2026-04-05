use crate::proxy::one_request_with_payload::process_one_request_with_payload;
use crate::proxy::wildcard::QueryParams;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use dir_and_db_pool::db::DbBool;
use log::error;
use serde_json::Value;

/// HANDLES ALL WILDCARD REQUESTS
pub async fn handle_wildcard_with_payload(
    pool: web::Data<DbBool>,
    req: HttpRequest,
    //  stream: web::Payload,
    query: web::Query<QueryParams>,
    body: web::Bytes,
) -> impl Responder {
    // println!("payload 1 {:?}", &body);

    match process_one_request_with_payload(pool, req, query, body).await {
        Ok(result) => result,
        Err(err) => {
            error!("Centrale error: {}", err);
            HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Not authenticated" }))
        }
    }
}
