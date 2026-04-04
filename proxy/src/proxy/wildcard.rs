use crate::proxy::one_request::process_one_request;
use actix_http::Request;
use actix_web::http::header;
use actix_web::{HttpRequest, web};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QueryParams {
    pub air_token: Option<String>,
}

/// HANDLES ALL WILDCARD REQUESTS
pub async fn handle_wildcard(
    pool: web::Data<DbBool>,
    req: HttpRequest,
    stream: web::Payload,
    query: web::Query<QueryParams>,
) -> impl Responder {
    match process_one_request(pool, req, stream, query).await {
        Ok(result) => result,
        Err(err) => {
            error!("Centrale error: {}", err);
            HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Not authenticated" }))
        }
    }
}

// TBD
// TBD CERTAIN URLS (.git, .env) SEND STRAIGHT TO RATE LIMITER
// TBD SET HEADERS Access-Control-Allow-Origin

// DONE:
// RATE LIMIT BY IP
// BROADCAST LOG
// GET COOKIE
// GET BEARER TOKEN
// AUTHENTICATE
// AUTHORIZE
// PROXY:
// // ADD TO REQ HEADERS
// // MAKE REQUEST
// // AWAIT
// // RESPOND TO CLIENT
// BROADCAST LOG
// PROXY SOCKET
// // DELETE SUBDOMAIN

use actix_web::{HttpResponse, Responder};
use dir_and_db_pool::db::DbBool;
use log::error;

#[actix_rt::test]
async fn test_empty_host_header() {
    use crate::proxy::create_test_app::_create_test_app;
    use actix_web::test;

    let app = _create_test_app().await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    println!("{:?}", &resp.status());
    println!("{:?}", &resp);
    assert!(resp.status().is_client_error());
}

pub fn _create_wildcard_request_with_host(cookie: String, host: String) -> Request {
    test::TestRequest::get()
        .uri("/")
        .insert_header((header::COOKIE, cookie))
        .insert_header(("Host", host))
        .insert_header(("Content-Type", "application/json"))
        .to_request()
}

pub fn _create_wildcard_request_with_referer(cookie: String, referer: &str) -> Request {
    test::TestRequest::get()
        .uri("/")
        .insert_header((header::COOKIE, cookie))
        .insert_header(("Referer", referer))
        .insert_header(("Content-Type", "application/json"))
        .to_request()
}

use actix_web::test;
//use serde_json::Value;
use serde_json::json;

pub fn _user_create_request() -> Request {
    let payload = json!({
        "username": "testuser",
        "password": "testpassword"
    });

    test::TestRequest::post()
        .uri("/api/user")
        .insert_header(("Content-Type", "application/json"))
        .set_json(&payload)
        .to_request()
}

#[actix_rt::test]
async fn has_referrer_ok() {
    use crate::proxy::subdomain::_one_wildcard_test_case_with_referer;

    let auth_resp = _one_wildcard_test_case_with_referer("http://subdomain.localhost.com").await;
    assert!(auth_resp.status().is_success());
}

#[actix_rt::test]
async fn has_host_ok() {
    use crate::proxy::subdomain::_one_wildcard_test_case_with_host;

    let auth_resp = _one_wildcard_test_case_with_host("https://hello.hello.ee").await;
    assert!(auth_resp.status().is_success());
}

#[actix_rt::test]
async fn has_one_work_host_err() {
    use crate::proxy::subdomain::_one_wildcard_test_case_with_host;

    let auth_resp = _one_wildcard_test_case_with_host("Some").await;
    assert!(auth_resp.status().is_client_error());
}
