use crate::config::CentraleConfig;
use actix_http::Request;
use actix_web::{
    Error, HttpResponse, Responder,
    cookie::Cookie,
    dev::{Service, ServiceResponse},
    web,
};
use dir_and_db_pool::db::DbBool;
//
pub async fn get_user(_pool: web::Data<DbBool>) -> impl Responder {
    HttpResponse::Ok().body("OK")
}

use actix_web::test;

pub async fn _make_get_user_request(
    cookie: &String,
    app: impl Service<Request, Response = ServiceResponse, Error = Error>,
) -> ServiceResponse {
    let baked_cookie = Cookie::build("my_cookie", cookie)
        .domain(CentraleConfig::DOMAIN)
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
    use crate::user::register::_create_test_pool;
    use crate::user::register::_create_test_user_register_app;

    let pool = _create_test_pool();
    let app = _create_test_user_register_app(pool).await;
    let resp = _make_get_user_request(&"cookie".to_string(), app).await;
    assert!(resp.status().is_success());
}
