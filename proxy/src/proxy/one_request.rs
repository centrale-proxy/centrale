use crate::{
    error::CentraleError,
    proxy::{
        authenticate_and_authorize::authenticate_and_authorize, is_ws::is_streaming_request,
        proxy_ws::ws_proxy,
    },
};
use actix_http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, web};
use config::CentraleConfig;
use dir_and_db_pool::db::DbBool;
use reqwest::header;

/// Process one wildcard request
pub async fn process_one_request(
    pool: web::Data<DbBool>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, CentraleError> {
    if is_streaming_request(&req) {
        let socket = ws_proxy(req, stream).await?;
        Ok(socket)
    } else {
        let (_user_id, subdomain, subdomain_user_role, pass, url) =
            authenticate_and_authorize(pool, req)?;

        // PROXY
        let client = reqwest::Client::new();
        let master_token = CentraleConfig::MASTER_BEARER_TOKEN;
        let response = client
            .get(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", master_token))
            .header("centrale_subdomain", format!("{}", subdomain))
            .header("centrale_password", format!("{}", pass))
            .header("centrale_role", format!("{}", subdomain_user_role))
            .send()
            .await?;

        let status = response.status();
        let body = response.bytes().await?;
        let res = HttpResponse::build(StatusCode::from_u16(status.as_u16()).unwrap()).body(body);

        Ok(res)
    }
}
