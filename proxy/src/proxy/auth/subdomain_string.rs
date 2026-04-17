use crate::{
    error::CentraleError,
    proxy::wildcard::test::{
        _create_wildcard_request_with_host, _create_wildcard_request_with_referer,
        _user_create_request,
    },
};
use actix_http::header::HeaderMap;
use actix_web::dev::ServiceResponse;
use config::CentraleConfig;
//use url::Url;
/*
pub fn get_subdomain_string(url: &str) -> Result<String, CentraleError> {
    let parsed_url = Url::parse(url)?;
    let host = parsed_url.host_str();
    match host {
        Some(host) => {
            let parts: Vec<&str> = host.split('.').collect();
            if parts.len() == 3 {
                let domain = format!("{}.{}", parts[1], parts[2]);
                if domain == CentraleConfig::get("DOMAIN") {
                    Ok(parts[0].to_string())
                } else {
                    Err(CentraleError::InvalidDomain)
                }
            } else {
                Err(CentraleError::InvalidSubdomain)
            }
        }
        None => Err(CentraleError::MissingHost),
    }
}
 */
#[actix_rt::test]
async fn empty_subdomain_error() {
    let auth_resp = _one_wildcard_test_case_with_host("").await;
    assert!(auth_resp.status().is_client_error());
}
/*
 // TBD NNEDS SUBDMOAIN CREATED
#[actix_rt::test]
async fn normal_subdomain_2() {
    let auth_resp = _one_wildcard_test_case_with_host("https://hello.hello.ee").await;
    assert!(auth_resp.status().is_success());
}
 */
#[actix_rt::test]
async fn domain_with_two_subdomains_fails() {
    let auth_resp = _one_wildcard_test_case_with_host("https://hello.hello.hello.ee").await;
    assert!(auth_resp.status().is_client_error());
}

#[actix_rt::test]
async fn just_domain_without_wildcard_fails() {
    let auth_resp = _one_wildcard_test_case_with_host("https://hello.ee").await;
    assert!(auth_resp.status().is_client_error());
}

pub fn _get_centrale_cookie(headers: &HeaderMap) -> Result<String, CentraleError> {
    let cookie_header = headers.get("set-cookie").unwrap().to_str().unwrap();
    let cookie = cookie_header.split(';').next().unwrap(); // Split by ';' and take the first part
    let cookie_value = cookie.split('=').nth(1).unwrap(); // Split by '=' and take the second part
    let value = cookie_value.to_string();
    Ok(value)
}

pub async fn _one_wildcard_test_case_with_host(host: &str) -> ServiceResponse {
    use actix_web::test;
    // DB AND SERVER
    use crate::proxy::test::create_test_app::_create_test_app;

    let app = _create_test_app().await;

    // CREATE USER
    let req = _user_create_request(host);
    let resp = test::call_service(&app, req).await;
    // GET COOKIE
    let cookie_value = _get_centrale_cookie(resp.headers()).unwrap();
    let cookie = format!("centrale={}", cookie_value);
    // MAKE WILDCARD REQUEST WITH COOKIE
    let wild_req = _create_wildcard_request_with_host(cookie, host.to_string());
    //
    let auth_resp = test::call_service(&app, wild_req).await;
    //
    auth_resp
}

pub async fn _one_wildcard_test_case_with_referer(referer: &str) -> ServiceResponse {
    use crate::user::post::test::_create_test_pool;
    use crate::user::post::test::_create_test_user_register_app;
    use actix_web::test;

    // DB AND SERVER
    let db = _create_test_pool();
    let app = _create_test_user_register_app(db).await;
    // CREATE USER
    let host = CentraleConfig::get("SAMPLE_SERVER_ADDRESS");
    let host_s = format!("https://{}", host);
    let req = _user_create_request(&host_s);
    let resp = test::call_service(&app, req).await;
    // GET COOKIE
    let cookie_value = _get_centrale_cookie(resp.headers()).unwrap();
    let cookie = format!("centrale={}", cookie_value);
    // MAKE WILDCARD REQUEST WITH COOKIE
    let wild_req = _create_wildcard_request_with_referer(cookie, referer);
    //
    let auth_resp = test::call_service(&app, wild_req).await;
    //
    auth_resp
}
