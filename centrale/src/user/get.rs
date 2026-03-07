use crate::{config::CentraleConfig, user::register::_create_test_user_register_app};
use actix_http::Request;
use actix_web::{
    App, Error, HttpResponse, Responder,
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
/*
pub async fn _create_get_user_app(
    pool: DbBool,
) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .route("/api/user", web::get().to(get_user)),
    )
    .await;
    app
}
 */
#[actix_rt::test]
async fn get_user_not_authenticated() {
    use crate::user::register::_create_test_pool;
    let pool = _create_test_pool();
    let app = _create_test_user_register_app(pool).await;
    // let resp = _make_user_register_test_request(payload, app).await;
    let resp = _make_get_user_request(&"cookie".to_string(), app).await;

    println!("{:?}", resp);
    assert!(resp.status().is_success());
    // assert!(resp.headers().contains_key("set-cookie"));
}
