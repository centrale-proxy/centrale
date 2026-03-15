use crate::db::init::init_db;
use crate::routes::routes;
use actix_http::Request;
use actix_web::{App, test};
use actix_web::{
    Error,
    dev::{Service, ServiceResponse},
    http::header,
    web,
};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde_json::Value;

pub fn _create_test_pool() -> Pool<SqliteConnectionManager> {
    let manager = SqliteConnectionManager::memory();
    let pool = Pool::new(manager).expect("Failed to create pool.");
    init_db(&pool).unwrap();
    pool
}

pub async fn _create_test_user_register_app(
    pool: Pool<SqliteConnectionManager>,
) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    let app = test::init_service(App::new().configure(routes).app_data(web::Data::new(pool))).await;
    app
}

pub async fn _make_user_register_test_request(
    payload: Value,
    app: &impl Service<Request, Response = ServiceResponse, Error = Error>,
) -> ServiceResponse {
    let req = test::TestRequest::post()
        .uri("/api/user")
        .insert_header(("Content-Type", "application/json"))
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(app, req).await;
    resp
}

pub async fn _make_request_with_cookie_to_wildcard(
    app: &impl Service<Request, Response = ServiceResponse, Error = Error>,
    cookie: &str,
) -> ServiceResponse {
    println!("cookie {:?}", &cookie);
    let req = test::TestRequest::get()
        .uri("/")
        .insert_header((header::COOKIE, cookie))
        .insert_header(("Referer", "http://subdomain.localhost.com"))
        .insert_header(("Content-Type", "application/json"))
        .to_request();

    let resp = test::call_service(app, req).await;
    resp
}

pub async fn _make_request_with_cookie(
    app: &impl Service<Request, Response = ServiceResponse, Error = Error>,
    cookie: &str,
) -> ServiceResponse {
    let req = test::TestRequest::get()
        .uri("/api/user")
        //  .insert_header(("Content-Type", "application/json"))
        .insert_header(("Cookie", cookie))
        .to_request();

    let resp = test::call_service(&app, req).await;
    resp
}

#[actix_rt::test]
async fn post_new_user() {
    use crate::proxy::create_test_app::_create_test_app;
    use serde_json::json;

    let app = _create_test_app().await;
    let payload = json!({
        "username": "testuser",
        "password": "testpassword"
    });
    let resp = _make_user_register_test_request(payload, &app).await;
    // println!("{:?}", resp);
    assert!(resp.status().is_success());
    assert!(resp.headers().contains_key("set-cookie"));
}

#[actix_rt::test]
async fn post_user_get_user_with_cookie() {
    use crate::proxy::create_test_app::_create_test_app;
    use serde_json::json;

    let app = _create_test_app().await;
    let payload = json!({
        "username": "testuser",
        "password": "testpassword"
    });
    let resp = _make_user_register_test_request(payload, &app).await;

    assert!(resp.status().is_success());
    assert!(resp.headers().contains_key("set-cookie"));

    let cookie_header = resp.headers().get("set-cookie").unwrap();
    let cookie = cookie_header.to_str().unwrap();

    let auth_resp = _make_request_with_cookie(&app, cookie).await;
    assert!(auth_resp.status().is_success());
}
