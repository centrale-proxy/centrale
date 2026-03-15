use actix_http::Request;
use actix_web::{
    Error, HttpRequest, HttpResponse, Responder,
    cookie::Cookie,
    dev::{Service, ServiceResponse},
    web,
};
use config::CentraleConfig;
use dir_and_db_pool::db::DbBool;
//
pub fn get_user_process(
    pool: web::Data<DbBool>,
    req: HttpRequest,
) -> Result<HttpResponse, CentraleError> {
    let headers = req.headers();
    let user_id = get_user_id(pool, headers, req.cookie("centrale"))?;
    let resp = HttpResponse::Ok().json(serde_json::json!({ "user_id": user_id.to_string() }));
    Ok(resp)
}

pub async fn get_user(pool: web::Data<DbBool>, req: HttpRequest) -> impl Responder {
    match get_user_process(pool, req) {
        Ok(result) => result,
        Err(err) => {
            error!("Get user error: {}", err);
            HttpResponse::UnprocessableEntity()
                .json(serde_json::json!({ "error": "Cannot get user" }))
        }
    }
}

use actix_web::test;
use log::error;

use crate::{
    error::{self, CentraleError},
    proxy::get_user_id::get_user_id,
};

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
    assert!(resp.status().is_client_error());
}
