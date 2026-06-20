pub mod create_client;
pub mod is_front;
pub mod one_request;
pub mod one_request_with_payload;
pub mod serve_front;
pub mod test;

use crate::{proxy::wildcard::one_request::process_one_request, server::auth::CentraleUser};
use actix_web::{HttpRequest, HttpResponse, Responder, dev::ConnectionInfo, web};
use dir_and_db_pool::db::DbPool;
use log::error;

/// HANDLES ALL WILDCARD REQUESTS. Responds to client
pub async fn handle_wildcard(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    stream: web::Payload,
    client: web::Data<reqwest::Client>,
    user: CentraleUser,
    conn: ConnectionInfo,
) -> impl Responder {
    match process_one_request(pool, req, stream, client, user, conn).await {
        Ok(result) => result,
        Err(err) => {
            error!("Centrale wildcard error: {}", err);
            HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Not authenticated" }))
        }
    }
}
