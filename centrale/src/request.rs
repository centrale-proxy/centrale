use crate::proxy::one_request::process_one_request;
use actix_web::HttpRequest;

/// HANDLES ALL WILDCARD REQUESTS
pub async fn handle_wildcard(req: HttpRequest) -> impl Responder {
    match process_one_request(req) {
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
use log::error;

#[actix_rt::test]
async fn test_empty_host_header() {
    //
    use actix_web::{App, test, web};
    let app = test::init_service(App::new().route("/", web::get().to(handle_wildcard))).await;
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_client_error());
    let body = test::read_body(resp).await;
    let expected_body =
        serde_json::to_string(&serde_json::json!({ "error": "Not authenticated" })).unwrap();
    assert_eq!(body, expected_body);
}

#[actix_rt::test]
async fn has_referrer_ok() {
    //
    use actix_web::{App, test, web};
    let app = test::init_service(App::new().route("/", web::get().to(handle_wildcard))).await;
    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("Referer", "https://hello.hello.ee"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn has_host_ok() {
    //
    use actix_web::{App, test, web};
    let app = test::init_service(App::new().route("/", web::get().to(handle_wildcard))).await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("Host", "https://hello.hello.ee"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn has_one_work_host_err() {
    //
    use actix_web::{App, test, web};
    let app = test::init_service(App::new().route("/", web::get().to(handle_wildcard))).await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("Host", "Some"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}
