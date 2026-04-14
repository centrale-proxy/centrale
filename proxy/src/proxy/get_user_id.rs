use crate::{
    error::CentraleError,
    user::{
        bearer_token::find_by_token::find_user_by_token,
        cookie::find_by_cookie::find_user_by_cookie,
    },
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
        let token_string = token
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .unwrap_or("none");
        // let token_string = token.unwrap().to_str()?;
        let user = find_user_by_token(&pool, &token_string)?;
        Ok(user)
    } else if cookie.is_some() {
        // COOKIE
        let cookie_value = match cookie {
            Some(cookie_string) => {
                let cookie_string_owned = cookie_string.to_string();
                let cookie_headers = cookie_string_owned.split(';').next().unwrap_or("none");
                let cookie_value = cookie_headers
                    .split('=')
                    .nth(1)
                    .unwrap_or("none")
                    .to_string();
                cookie_value
            }
            None => {
                return Err(CentraleError::NoCookie);
            }
        };
        let user = find_user_by_cookie(&pool, &cookie_value)?;
        Ok(user)
    } else {
        // NO AUTH
        Err(CentraleError::NoTokenOrCookiePresent)
    }
}

#[actix_rt::test]
async fn fails_without_cookie_and_token() {
    use crate::proxy::create_test_app::_create_test_app;
    use actix_web::test;

    dotenvy::dotenv().ok();
    let app = _create_test_app().await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("Host", "https://hello.hello.ee"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_rt::test]
async fn random_token_does_not_work() {
    use crate::proxy::create_test_app::_create_test_app;
    use actix_web::http::header::AUTHORIZATION;
    use actix_web::{http::header::HeaderValue, test};

    dotenvy::dotenv().ok();
    let app = _create_test_app().await;

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
    use crate::proxy::create_test_app::_create_test_app;
    use actix_web::{http::header::COOKIE, test};

    dotenvy::dotenv().ok();
    let app = _create_test_app().await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("Host", "https://hello.hello.ee"))
        .insert_header((COOKIE, format!("centrale={}", "your_cookie_value_here")))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}
