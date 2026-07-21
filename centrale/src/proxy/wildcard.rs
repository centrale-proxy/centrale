pub mod create_client;
pub mod one_request;
pub mod one_request_with_payload;
pub mod serve_front;
pub mod test;

use crate::{proxy::wildcard::one_request::process_one_request, server::auth::CentraleUser};
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use log::error;

/// HANDLES ALL WILDCARD REQUESTS. Responds to client
pub async fn handle_wildcard(
    req: HttpRequest,
    stream: web::Payload,
    client: web::Data<reqwest::Client>,
    user: CentraleUser,
) -> impl Responder {
    match process_one_request(req, stream, client, user).await {
        Ok(result) => result,
        Err(err) => {
            error!("Centrale wildcard error: {}", err);
            HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Not authenticated" }))
        }
    }
}
