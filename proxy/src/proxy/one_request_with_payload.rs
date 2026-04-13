use std::str::FromStr;

use crate::{
    error::CentraleError,
    proxy::{authenticate_and_authorize::authenticate_and_authorize, wildcard::QueryParams},
};
use actix_http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, web};
use config::CentraleConfig;
use dir_and_db_pool::db::DbBool;
use reqwest::{Method, header};
use serde_json::Value;

/// Process one wildcard request
pub async fn process_one_request_with_payload(
    pool: web::Data<DbBool>,
    req: HttpRequest,
    //  stream: web::Payload,
    _query: web::Query<QueryParams>,
    body: web::Bytes,
) -> Result<HttpResponse, CentraleError> {
    //println!("body {:?}", &body);
    // IS NORMAL
    let (_user_id, subdomain, subdomain_user_role, pass, url) =
        authenticate_and_authorize(pool, &req)?;
    // PROXY
    let client = reqwest::Client::new();
    let master_token = CentraleConfig::master_bearer_token();
    //let method = req.method();
    let method = Method::from_str(req.method().as_str()).unwrap();
    //println!("method: {:?}", method);
    let mut request = client
        .request(method.clone(), url)
        .header(header::AUTHORIZATION, format!("Bearer {}", master_token))
        .header("centrale_subdomain", format!("{}", subdomain))
        .header("centrale_password", format!("{}", pass))
        .header("centrale_role", format!("{}", subdomain_user_role));
    // .json(&payload)
    //.send()
    //.await?;
    //
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
