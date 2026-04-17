use crate::{error::CentraleError, proxy::wildcard::QueryParams, server::auth::CentraleUser};
use actix_http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, web};
use config::CentraleConfig;
use reqwest::{Method, header};
use serde_json::Value;
use std::str::FromStr;

/// Process one wildcard request
pub async fn process_one_request_with_payload(
    req: HttpRequest,
    _query: web::Query<QueryParams>,
    body: web::Bytes,
    client: web::Data<reqwest::Client>,
    user: CentraleUser,
) -> Result<HttpResponse, CentraleError> {
    let master_token = CentraleConfig::master_bearer_token();
    let is_method = Method::from_str(req.method().as_str());
    let method = match is_method {
        Ok(method) => method,
        Err(_err) => {
            return Err(CentraleError::InvalidMethod);
        }
    };

    let https = format!("https://{}", user.url);
    let mut request = client
        .request(method.clone(), https)
        .header(header::AUTHORIZATION, format!("Bearer {}", master_token))
        .header("centrale_subdomain", format!("{}", user.subdomain))
        .header("centrale_password", format!("{}", user.pass))
        .header("centrale_role", format!("{}", user.role));

    let payload: Value = if body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&body).unwrap_or(Value::Null)
    };

    if matches!(method, Method::POST | Method::PUT) {
        request = request.json(&payload);
    }

    let response = request.send().await?;

    let status = response.status();
    let body = response.bytes().await?;
    let res = HttpResponse::build(StatusCode::from_u16(status.as_u16()).unwrap()).body(body);

    Ok(res)
}
