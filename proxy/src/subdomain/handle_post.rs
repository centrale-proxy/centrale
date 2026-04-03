use crate::{
    error::CentraleError, proxy::get_user_id::get_user_id, subdomain::post::post_subdomain,
};
use actix_http::Request;
use actix_web::{
    HttpRequest, HttpResponse,
    dev::{Service, ServiceResponse},
    http::header,
    web,
};
use dir_and_db_pool::db::DbBool;
use log::error;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct RegisterSubdomain {
    pub subdomain: String,
}

pub fn handle_post(
    pool: web::Data<DbBool>,
    json: web::Json<RegisterSubdomain>,
    req: HttpRequest,
) -> Result<HttpResponse, CentraleError> {
    let subdomain = json.subdomain.clone();
    let headers = req.headers();
    let user_id = get_user_id(pool.clone(), headers, req.cookie("centrale"))?;
    let db = pool.get().expect("Couldn't get db connection from pool");

    match post_subdomain(&db, &subdomain, user_id) {
        Ok(result) => {
            // TBD SEND TO DESTINATION SERVER
            //
            let res = HttpResponse::Ok()
                .json(serde_json::json!({ "subdomain": result, "user": user_id }));
            Ok(res)
        }
        Err(err) => {
            error!("Add subdomain error: {}", err);
            Ok(HttpResponse::UnprocessableEntity()
                .json(serde_json::json!({ "error": "Cannot add subdomain" })))
        }
    }
}

use actix_web::Error;
use actix_web::test;

pub async fn _make_register_subdomain_request(
    payload: Value,
    app: &impl Service<Request, Response = ServiceResponse, Error = Error>,
    cookie: &str,
) -> ServiceResponse {
    let req = test::TestRequest::post()
        .uri("/api/subdomain")
        .insert_header(("Content-Type", "application/json"))
        .insert_header((header::COOKIE, cookie))
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(app, req).await;
    resp
}

#[actix_rt::test]
async fn post_subdomain_normal() {
    use crate::user::post::test::{_make_request_with_cookie, _make_user_register_test_request};
    use serde_json::json;

    use crate::proxy::create_test_app::_create_test_app;

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

    let register_subdomain_payload = json!({
        "subdomain": "test",
    });

    let sub_reg = _make_register_subdomain_request(register_subdomain_payload, &app, cookie).await;

    assert!(sub_reg.status().is_success());
}
