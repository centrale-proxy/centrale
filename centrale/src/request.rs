use actix_web::HttpRequest;

pub async fn handle_request(req: HttpRequest) -> impl Responder {
    let host = req.headers().get("Host").and_then(|h| h.to_str().ok());
    let referer = req.headers().get("Referer");
    if host.is_some() {
        "yes"
    } else if referer.is_some() {
        // CLOUDFLARE HAS NO HOST, ONLY REFERRER
        "yes"
    } else {
        HttpResponse::Unauthorized().json(serde_json::json!({ "error": "No authentication" }));
        "No"
    }
}

use actix_web::{App, HttpResponse, Responder, test, web};

#[actix_rt::test]
async fn test_empty_host_header() {
    //
    let app = test::init_service(App::new().route("/", web::get().to(handle_request))).await;

    let req = test::TestRequest::get()
        .uri("/")
        //.insert_header(("Host", ""))
        .to_request();

    let resp = test::call_service(&app, req).await;

    //assert!(resp.status().is_client_error());
    let body = test::read_body(resp).await;
    assert_eq!(body, "No");
}
