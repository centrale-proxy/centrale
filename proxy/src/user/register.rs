use crate::{
    error::CentraleError,
    request::handle_wildcard,
    routes::routes,
    subdomain::respond_post::respond_subdomain,
    user::{add::add_user, cookie::add_cookie, get::get_user},
};
use actix_http::Request;
use actix_web::{
    Error, HttpResponse,
    cookie::{Cookie, time::Duration},
    dev::{Service, ServiceResponse},
    http::header,
    web,
};
use config::CentraleConfig;
use dir_and_db_pool::db::DbBool;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub password: String,
}

pub fn handle_register(
    pool: web::Data<DbBool>,
    json: web::Json<RegisterUser>,
) -> Result<HttpResponse, CentraleError> {
    let register_request = json.into_inner();
    let username = register_request.username;
    let password = register_request.password;
    let db = pool.get().expect("Couldn't get db connection from pool");
    let user_id = add_user(&db, &username, &password)?;
    let cookie_value = add_cookie(&db, user_id)?;
    // DO COOKIE
    let cookie = Cookie::build("centrale", cookie_value)
        .domain(CentraleConfig::DOMAIN)
        .max_age(Duration::new(CentraleConfig::COOKIE_TIMEOUT, 0))
        .secure(CentraleConfig::COOKIE_SECURE) // Only send over HTTPS
        .http_only(CentraleConfig::COOKIE_HTTP_ONLY) // Not accessible via JavaScript
        // .path("/")
        .finish();
    // ADD COOKIE
    let resp = HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({ "user_id": user_id.to_string() }));

    Ok(resp)
}

use crate::db::init::init_db;
use crate::user::post::post_user;
use actix_web::{App, test};
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
    use serde_json::json;
    let pool = _create_test_pool();
    let app = _create_test_user_register_app(pool).await;
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
    use serde_json::json;
    let pool = _create_test_pool();
    let app = _create_test_user_register_app(pool).await;
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
