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
        let token_option = token
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "));

        match token_option {
            Some(token_string) => {
                let user = find_user_by_token(&pool, &token_string)?;
                Ok(user)
            }
            None => return Err(CentraleError::InvalidToken),
        }
    } else if cookie.is_some() {
        // COOKIE
        match &cookie {
            Some(cookie) => {
                let co = cookie.name().to_owned();
                if co == "centrale" {
                    let cookie_value = cookie.value().to_string();
                    let user = find_user_by_cookie(&pool, &cookie_value)?;
                    Ok(user)
                } else {
                    Err(CentraleError::NoTokenOrCookiePresent)
                }
            }
            None => Err(CentraleError::NoTokenOrCookiePresent),
        }
    } else {
        // NO AUTH
        Err(CentraleError::NoTokenOrCookiePresent)
    }
}

#[actix_rt::test]
async fn fails_without_cookie_and_token() {
    use crate::proxy::test::create_test_app::_create_test_app;
    use actix_web::test;

    dotenvy::dotenv().ok();
    let app = _create_test_app().await;

    let req = test::TestRequest::get().uri("/").to_request();

    let resp = test::try_call_service(&app, req).await.unwrap();

    assert!(resp.status().is_client_error());
}

#[actix_rt::test]
async fn random_token_does_not_work() {
    use crate::proxy::test::create_test_app::_create_test_app;
    use actix_web::http::header::AUTHORIZATION;
    use actix_web::{http::header::HeaderValue, test};

    dotenvy::dotenv().ok();
    let app = _create_test_app().await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header((
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", "token")).unwrap(),
        ))
        .to_request();

    let resp = test::try_call_service(&app, req).await.unwrap();
    assert!(resp.status().is_client_error());
}

#[actix_rt::test]
async fn random_cookie_not_working() {
    use crate::proxy::test::create_test_app::_create_test_app;
    use actix_web::{http::header::COOKIE, test};

    dotenvy::dotenv().ok();
    let app = _create_test_app().await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header((COOKIE, format!("centrale={}", "your_cookie_value_here")))
        .to_request();

    let resp = test::try_call_service(&app, req).await.unwrap();
    assert!(resp.status().is_client_error());
}
