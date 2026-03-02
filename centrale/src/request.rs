use actix_web::HttpRequest;

pub async fn handle_request(req: HttpRequest) -> impl Responder {
    let host = req.headers().get("Host");
    let referer = req.headers().get("Referer");
    // TBD RATE LIMIT BY IP
    // TBD CERTAIN URLS (.git, .env) SEND STRAIGHT TO RATE LIMITER
    // TBD BROADCAST LOG
    if host.is_some() {
        let subdomain = get_subdomain(host.unwrap());
        match subdomain {
            Ok(subdomain) => {
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
                HttpResponse::Ok().json(serde_json::json!({ "Ok": subdomain }))
            }
            Err(err) => {
                error!("{err}");
                HttpResponse::Unauthorized()
                    .json(serde_json::json!({ "error": "Not authenticated" }))
            }
        }
    } else if referer.is_some() {
        // CLOUDFLARE HAS NO HOST, ONLY REFERRER
        let subdomain = format!("{:?}", host);
        HttpResponse::Ok().json(serde_json::json!({ "Ok": subdomain }))
    } else {
        HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Not authenticated" }))
    }
}

use crate::subdomain::get_subdomain;
use actix_web::{HttpResponse, Responder};
use log::error;

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
        .insert_header(("Host", "https://hello.hello.ee"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn has_one_work_host_err() {
    //
    use actix_web::{App, test, web};
    let app = test::init_service(App::new().route("/", web::get().to(handle_request))).await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("Host", "Some"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}
