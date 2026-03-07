use crate::error::CentraleError;
use actix_web::http::header::HeaderValue;
use url::Url;

pub fn get_subdomain(input_url: &HeaderValue) -> Result<String, CentraleError> {
    if let Ok(url) = input_url.to_str() {
        let parsed_url = Url::parse(url)?;
        let host = parsed_url.host_str();
        match host {
            Some(host) => {
                let parts: Vec<&str> = host.split('.').collect();
                if parts.len() == 3 {
                    // IF HOST HAS 3 PARTS RETURN THE FIRST
                    Ok(parts[0].to_string())
                } else {
                    Err(CentraleError::InvalidSubdomain)
                }
            }
            None => Err(CentraleError::MissingHost),
        }
    } else {
        Err(CentraleError::UnableToParseUrl)
    }
}

#[actix_rt::test]
async fn empty_subdomain_error() {
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
    use actix_web::http::header::AUTHORIZATION;
    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("Host", ""))
        .insert_header((
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", "token")).unwrap(),
        ))
        .to_request();

    let resp = test::call_service(&app, req).await;
    println!("resp {:?}", &resp);
    assert!(resp.status().is_client_error());
}

#[actix_rt::test]
async fn normal_subdomain() {
    use crate::request::handle_wildcard;
    use crate::user::register::_create_test_pool;
    use actix_web::http::header::AUTHORIZATION;
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
        .insert_header((
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", "token")).unwrap(),
        ))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn domain_with_two_subdomains_fails() {
    use crate::request::handle_wildcard;
    use crate::user::register::_create_test_pool;
    use actix_web::http::header::AUTHORIZATION;
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
        .insert_header(("Host", "https://hello.hello.hello.ee"))
        .insert_header((
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", "token")).unwrap(),
        ))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_rt::test]
async fn just_domain_without_wildcard_fails() {
    use crate::request::handle_wildcard;
    use crate::user::register::_create_test_pool;
    use actix_web::http::header::AUTHORIZATION;
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
        .insert_header(("Host", "https://hello.ee"))
        .insert_header((
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", "token")).unwrap(),
        ))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}
