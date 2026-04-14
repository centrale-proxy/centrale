use actix_http::Request;
use actix_web::test;
use actix_web::{
    Error,
    cookie::Cookie,
    dev::{Service, ServiceResponse},
};
use config::CentraleConfig;

pub async fn _make_get_user_request(
    cookie: &String,
    app: impl Service<Request, Response = ServiceResponse, Error = Error>,
) -> ServiceResponse {
    let baked_cookie = Cookie::build("my_cookie", cookie)
        .domain(CentraleConfig::get("DOMAIN"))
        .path("/")
        .finish();

    let req = test::TestRequest::get()
        .uri("/api/user")
        .insert_header(("Content-Type", "application/json"))
        .cookie(baked_cookie)
        .to_request();

    let resp = test::call_service(&app, req).await;
    resp
}
#[actix_rt::test]
async fn get_user_not_authenticated() {
    use crate::proxy::create_test_app::_create_test_app;

    let app = _create_test_app().await;

    let resp = _make_get_user_request(&"cookie".to_string(), app).await;
    assert!(resp.status().is_client_error());
}
