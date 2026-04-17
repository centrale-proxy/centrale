use actix_http::Request;
use actix_web::http::header;

#[actix_rt::test]
async fn test_empty_host_header() {
    use crate::proxy::test::create_test_app::_create_test_app;
    use actix_web::test;

    let app = _create_test_app().await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
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
use serde_json::json;

pub fn _user_create_request(host: &str) -> Request {
    let payload = json!({
        "username": "testuser",
        "password": "testpassword"
    });

    test::TestRequest::post()
        .uri("/api/user_add")
        .insert_header(("Host", host))
        .insert_header(("Content-Type", "application/json"))
        .set_json(&payload)
        .to_request()
}
/*
 //TBD NEEDS SUBDOMAINS CREATED
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
 */
#[actix_rt::test]
async fn has_one_work_host_err() {
    use crate::proxy::auth::subdomain::_one_wildcard_test_case_with_host;

    let auth_resp = _one_wildcard_test_case_with_host("Some").await;
    assert!(auth_resp.status().is_client_error());
}
