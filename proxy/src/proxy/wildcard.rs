pub mod create_client;
pub mod one_request;
pub mod one_request_with_payload;
pub mod test;

use crate::{proxy::wildcard::one_request::process_one_request, server::auth::CentraleUser};
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use dir_and_db_pool::db::DbBool;
use log::error;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QueryParams {
    pub air_token: Option<String>,
}

/// HANDLES ALL WILDCARD REQUESTS. Responds to client
pub async fn handle_wildcard(
    pool: web::Data<DbBool>,
    req: HttpRequest,
    stream: web::Payload,
    //  query: web::Query<QueryParams>,
    client: web::Data<reqwest::Client>,
    user: CentraleUser,
) -> impl Responder {
    match process_one_request(pool, req, stream, client, user).await {
        Ok(result) => result,
        Err(err) => {
            error!("Centrale wildcard error: {}", err);
            HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Not authenticated" }))
        }
    }
}
