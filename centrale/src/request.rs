use crate::error::CentraleError;
use crate::proxy::one_request::process_one_request;
use crate::user::register::{
    _create_test_pool, _create_test_user_register_app, _make_request_with_cookie,
    _make_request_with_cookie_to_wildcard, _make_user_register_test_request,
};
use actix_http::Request;
use actix_http::header::HeaderMap;
use actix_web::http::header;
use actix_web::{HttpRequest, web};

/// HANDLES ALL WILDCARD REQUESTS
pub async fn handle_wildcard(pool: web::Data<DbBool>, req: HttpRequest) -> impl Responder {
    match process_one_request(pool, req) {
        Ok(result) => result,
        Err(err) => {
            error!("Centrale error: {}", err);
            HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Not authenticated" }))
        }
    }
}

// TBD
// TBD RATE LIMIT BY IP
// TBD CERTAIN URLS (.git, .env) SEND STRAIGHT TO RATE LIMITER
// TBD BROADCAST LOG
//
// GET COOKIE
// GET BEARER TOKEN
// AUTHENTICATE
// AUTHORIZE
// SET HEADERS Access-Control-Allow-Origin
// PROXY:
// // ADD TO REQ HEADERS
// // MAKE REQUEST
// // AWAIT
// // RESPOND TO CLIENT
// BROADCAST LOG
//
// // DELETE SUBDOMAIN

use actix_web::{HttpResponse, Responder};
use dir_and_db_pool::db::DbBool;
use log::error;

#[actix_rt::test]
async fn test_empty_host_header() {
    //
    use actix_web::{App, test, web};
    let db = _create_test_pool();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/", web::get().to(handle_wildcard)),
    )
    .await;
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    println!("{:?}", &resp.status());
    println!("{:?}", &resp);
    assert!(resp.status().is_client_error());
    /*
    let body = test::read_body(resp).await;
    let expected_body =
        serde_json::to_string(&serde_json::json!({ "error": "Not authenticated" })).unwrap();
    assert_eq!(body, expected_body);
    */
}

pub fn get_centrale_cookie(headers: &HeaderMap) -> Result<String, CentraleError> {
    let cookie_header = headers.get("set-cookie").unwrap().to_str().unwrap();
    let cookie = cookie_header.split(';').next().unwrap(); // Split by ';' and take the first part
    let cookie_value = cookie.split('=').nth(1).unwrap(); // Split by '=' and take the second part
    let value = cookie_value.to_string();
    Ok(value)
}

pub fn _create_wildcard_request_with_host(cookie: String, host: String) -> Request {
    test::TestRequest::get()
        .uri("/")
        .insert_header((header::COOKIE, cookie))
        .insert_header(("Host", host))
        .insert_header(("Content-Type", "application/json"))
        .to_request()
}

pub fn _create_wildcard_request(cookie: String) -> Request {
    test::TestRequest::get()
        .uri("/")
        .insert_header((header::COOKIE, cookie))
        .insert_header(("Referer", "http://subdomain.localhost.com"))
        .insert_header(("Content-Type", "application/json"))
        .to_request()
}

use actix_web::test;
use serde_json::Value;
use serde_json::json;

pub fn _user_create_request(payload: Value) -> Request {
    test::TestRequest::post()
        .uri("/api/user")
        .insert_header(("Content-Type", "application/json"))
        .set_json(&payload)
        .to_request()
}

#[actix_rt::test]
async fn has_referrer_ok() {
    // START POOL AND DB
    let pool = _create_test_pool();
    let app = _create_test_user_register_app(pool).await;
    // PAYLOAD
    let payload = json!({
        "username": "testuser",
        "password": "testpassword"
    });
    // CREATE USER
    let req = _user_create_request(payload);
    let resp = test::call_service(&app, req).await;
    // ASSERT REGISTRATION
    assert!(resp.status().is_success());
    assert!(resp.headers().contains_key("set-cookie"));
    // GET COOKIE
    let cookie_value = get_centrale_cookie(resp.headers()).unwrap();
    let cookie = format!("centrale={}", cookie_value);
    // MAKE WILDCARD REQUEST WITH COOKIE
    let wild_req = _create_wildcard_request(cookie);
    let auth_resp = test::call_service(&app, wild_req).await;
    assert!(auth_resp.status().is_success());
}

#[actix_rt::test]
async fn has_host_ok() {
    use actix_web::http::header::{AUTHORIZATION, HeaderValue};
    use actix_web::{App, test, web};

    let db = _create_test_pool();
    let app = _create_test_user_register_app(db).await;

    let payload = json!({
        "username": "testuser",
        "password": "testpassword"
    });
    // CREATE USER
    let req = _user_create_request(payload);
    let resp = test::call_service(&app, req).await;
    // ASSERT REGISTRATION
    assert!(resp.status().is_success());
    assert!(resp.headers().contains_key("set-cookie"));
    // GET COOKIE
    let cookie_value = get_centrale_cookie(resp.headers()).unwrap();
    let cookie = format!("centrale={}", cookie_value);
    // MAKE WILDCARD REQUEST WITH COOKIE
    let wild_req = _create_wildcard_request_with_host(cookie, "https://hello.hello.ee".to_string());
    let auth_resp = test::call_service(&app, wild_req).await;
    assert!(auth_resp.status().is_success());
}

#[actix_rt::test]
async fn has_one_work_host_err() {
    use actix_web::http::header::{AUTHORIZATION, HeaderValue};
    use actix_web::{App, test, web};
    let db = _create_test_pool();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/", web::get().to(handle_wildcard)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("Host", "Some"))
        .insert_header((
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", "token")).unwrap(),
        ))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}
