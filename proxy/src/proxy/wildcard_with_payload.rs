use crate::{
    proxy::wildcard::{QueryParams, one_request_with_payload::process_one_request_with_payload},
    server::auth::CentraleUser,
};
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use log::error;

/// HANDLES WILDCARD REQUESTS WITH PAYLOAD. Separate file because websocket and body are the same thing.
pub async fn handle_wildcard_with_payload(
    req: HttpRequest,
    query: web::Query<QueryParams>,
    body: web::Bytes,
    client: web::Data<reqwest::Client>,
    user: CentraleUser,
) -> impl Responder {
    match process_one_request_with_payload(req, query, body, client, user).await {
        Ok(result) => result,
        Err(err) => {
            error!("Centrale wildcard payload error: {}", err);
            HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Not authenticated" }))
        }
    }
}
