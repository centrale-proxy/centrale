use actix_web::HttpRequest;

pub async fn handle_request(req: HttpRequest) -> impl Responder {
    let host = req.headers().get("Host");
    let referer = req.headers().get("Referer");
    if host.is_some() {
        HttpResponse::Ok().json(serde_json::json!({ "Ok": true }))
    } else if referer.is_some() {
        // CLOUDFLARE HAS NO HOST, ONLY REFERRER
        HttpResponse::Ok().json(serde_json::json!({ "Ok": true }))
    } else {
        HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Not authenticated" }))
    }
}

use actix_web::{HttpResponse, Responder};

#[actix_rt::test]
async fn test_empty_host_header() {
    //
    use actix_web::{App, test, web};
    let app = test::init_service(App::new().route("/", web::get().to(handle_request))).await;
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
    let app = test::init_service(App::new().route("/", web::get().to(handle_request))).await;
    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("Referer", "Some"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn has_host_ok() {
    //
    use actix_web::{App, test, web};
    let app = test::init_service(App::new().route("/", web::get().to(handle_request))).await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("Host", "Some"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
