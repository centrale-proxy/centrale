use crate::{
    db::get_db::get_centrale_db, error::CentraleError, proxy::get_user_id::get_user_id,
    subdomain::post::post_subdomain,
};
use actix_http::Request;
use actix_web::{
    HttpRequest, HttpResponse,
    dev::{Service, ServiceResponse},
    web,
};
use common::truncate;
use config::CentraleConfig;
use dir_and_db_pool::db::DbBool;
use log::error;
use reqwest::header;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct RegisterSubdomain {
    pub subdomain: String,
}

pub async fn handle_post(
    pool: web::Data<DbBool>,
    payload: web::Json<RegisterSubdomain>,
    req: HttpRequest,
) -> Result<HttpResponse, CentraleError> {
    let subdomain_original = payload.subdomain.clone();
    let subdomain = truncate(&subdomain_original, 20);
    let headers = req.headers();
    let user_id = get_user_id(pool.clone(), headers, req.cookie("centrale"))?;

    let db = get_centrale_db(pool.get_ref())?;

    match post_subdomain(&db, &subdomain, user_id) {
        Ok(password) => {
            // SEND TO DESTINATION SERVER
            let client = reqwest::Client::new();
            let master_token = CentraleConfig::master_bearer_token();

            let url = format!(
                "{}/api/register_subdomain",
                CentraleConfig::SAMPLE_SERVER_ADDRESS
            );

            let mut map = HashMap::new();
            map.insert("hello", "hello");
            // println!("url {}", &url);
            let response = client
                .post(&url)
                .json(&map)
                .header(header::AUTHORIZATION, format!("Bearer {}", master_token))
                .header("centrale_subdomain", format!("{}", subdomain))
                .header("centrale_password", format!("{}", password))
                .header("centrale_role", format!("{}", "admin"))
                .send()
                .await;

            //  println!("response: {:?}", &response);

            let res = response.unwrap();

            let status = res.status();
            //  println!("status: {}", status);

            match status.as_u16() {
                200 => {
                    let res = HttpResponse::Ok()
                        .json(serde_json::json!({ "subdomain": subdomain, "user": user_id }));
                    Ok(res)
                }
                _ => Err(CentraleError::StringError("Wrong status".to_string())),
            }
        }
        Err(err) => {
            error!("Add subdomain error handle: {}", err);
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
    use actix_web::http::header;

    let req = test::TestRequest::post()
        .uri("/api/subdomain")
        .insert_header(("Content-Type", "application/json"))
        .insert_header((header::COOKIE, cookie))
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(app, req).await;
    resp
}
//
async fn _create_user_get_cookie(
    app: &impl Service<Request, Response = ServiceResponse, Error = Error>,
) -> String {
    use crate::user::post::test::_make_user_register_test_request;
    use serde_json::json;

    let payload = json!({
        "username": "testuser",
        "password": "testpassword"
    });

    let resp = _make_user_register_test_request(payload, &app).await;

    let cookie_header = resp.headers().get("set-cookie").unwrap();
    let cookie = cookie_header.to_str().unwrap();
    cookie.to_string()
}
#[actix_rt::test]
async fn post_subdomain_normal() {
    use crate::proxy::create_test_app::_create_test_app;
    use crate::user::post::test::_make_request_with_cookie;
    use serde_json::json;

    let app = _create_test_app().await;
    let cookie = _create_user_get_cookie(&app).await;

    let auth_resp = _make_request_with_cookie(&app, &cookie).await;
    assert!(auth_resp.status().is_success());

    let register_subdomain_payload = json!({
        "subdomain": "test",
    });

    let sub_reg = _make_register_subdomain_request(register_subdomain_payload, &app, &cookie).await;

    assert!(sub_reg.status().is_success());
}
#[actix_rt::test]
async fn post_subdomain_0_bytes_fails() {
    use crate::proxy::create_test_app::_create_test_app;
    use serde_json::json;

    let app = _create_test_app().await;
    let cookie = _create_user_get_cookie(&app).await;

    let register_subdomain_payload = json!({
        "subdomain": "\0",
    });

    let sub_reg = _make_register_subdomain_request(register_subdomain_payload, &app, &cookie).await;

    assert!(sub_reg.status().is_client_error());
}
#[actix_rt::test]
async fn post_subdomain_21_chars_cuts_to_20_chars() {
    use crate::proxy::create_test_app::_create_test_app;
    use actix_web::body::to_bytes;
    use serde_json::json;

    let app = _create_test_app().await;
    let cookie = _create_user_get_cookie(&app).await;

    let register_subdomain_payload = json!({
        "subdomain": "012345678901234567891",
    });

    let sub_reg = _make_register_subdomain_request(register_subdomain_payload, &app, &cookie).await;

    let body_bytes = to_bytes(sub_reg.into_body()).await.unwrap();
    let str = String::from_utf8(body_bytes.to_vec()).unwrap();

    let subdomain: RegisterSubdomain = serde_json::from_str(&str).unwrap();

    assert!(subdomain.subdomain.len() == 20);
}
