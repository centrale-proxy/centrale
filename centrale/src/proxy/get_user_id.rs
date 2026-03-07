use crate::{
    error::CentraleError,
    user::{find_by_cookie::find_user_by_cookie, find_by_token::find_user_by_token},
};
use actix_web::{
    cookie::Cookie,
    http::header::{AUTHORIZATION, HeaderMap},
    web,
};
use dir_and_db_pool::db::DbBool;

pub fn get_user_id(
    pool: web::Data<DbBool>,
    headers: &HeaderMap,
    cookie: Option<Cookie<'_>>,
) -> Result<i64, CentraleError> {
    //
    let token = headers.get(AUTHORIZATION);
    // PREFER TOKEN
    if token.is_some() {
        // BEARER TOKEN
        let token_string = token.unwrap().to_str()?;
        let user = find_user_by_token(&pool, &token_string.to_string())?;
        Ok(user)
    } else if cookie.is_some() {
        // COOKIE
        let cookie_string = cookie.unwrap().to_string();
        let user = find_user_by_cookie(&pool, &cookie_string)?;
        Ok(user)
    } else {
        // NO AUTH
        Err(CentraleError::NoTokenOrCookiePresent)
    }
}

#[actix_rt::test]
async fn fails_without_cookie_and_token() {
    use crate::request::handle_wildcard;
    use crate::user::register::_create_test_pool;
    use actix_web::{App, test, web};

    let db = _create_test_pool();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/", web::get().to(handle_wildcard)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("Host", "https://hello.hello.ee"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_rt::test]
async fn random_token_does_not_work() {
    use crate::request::handle_wildcard;
    use crate::user::register::_create_test_pool;
    use actix_web::http::header::AUTHORIZATION;
    use actix_web::{App, http::header::HeaderValue, test, web};

    let db = _create_test_pool();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/", web::get().to(handle_wildcard)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("Host", "https://hello.hello.ee"))
        .insert_header((
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", "token")).unwrap(),
        ))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_rt::test]
async fn random_cookie_not_working() {
    use crate::request::handle_wildcard;
    use crate::user::register::_create_test_pool;
    use actix_web::{App, http::header::COOKIE, test, web};

    let db = _create_test_pool();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/", web::get().to(handle_wildcard)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("Host", "https://hello.hello.ee"))
        .insert_header((COOKIE, format!("centrale={}", "your_cookie_value_here")))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}
