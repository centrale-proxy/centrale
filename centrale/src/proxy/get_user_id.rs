use crate::error::CentraleError;
use actix_web::{
    cookie::Cookie,
    http::header::{AUTHORIZATION, HeaderMap},
};

pub fn get_user_id(headers: &HeaderMap, cookie: Option<Cookie<'_>>) -> Result<i64, CentraleError> {
    //
    let token = headers.get(AUTHORIZATION);
    // PREFER TOKEN
    if token.is_some() {
        Ok(1)
    } else if cookie.is_some() {
        Ok(1)
    } else {
        // NO AUTH
        Err(CentraleError::NoTokenOrCookiePresent)
    }
}

#[actix_rt::test]
async fn fails_without_cookie_and_token() {
    use crate::request::handle_wildcard;
    use actix_web::{App, test, web};

    let app = test::init_service(App::new().route("/", web::get().to(handle_wildcard))).await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("Host", "https://hello.hello.ee"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_rt::test]
async fn works_with_token() {
    use crate::request::handle_wildcard;
    use actix_web::http::header::AUTHORIZATION;
    use actix_web::{App, http::header::HeaderValue, test, web};

    let app = test::init_service(App::new().route("/", web::get().to(handle_wildcard))).await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("Host", "https://hello.hello.ee"))
        .insert_header((
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", "token")).unwrap(),
        ))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn works_with_cookie() {
    use crate::request::handle_wildcard;
    use actix_web::{App, http::header::COOKIE, test, web};

    let app = test::init_service(App::new().route("/", web::get().to(handle_wildcard))).await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("Host", "https://hello.hello.ee"))
        .insert_header((COOKIE, format!("centrale={}", "your_cookie_value_here")))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
