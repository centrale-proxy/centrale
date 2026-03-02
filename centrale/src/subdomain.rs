use crate::error::CentraleError;
use actix_web::http::header::HeaderValue;

pub fn get_subdomain(value: &HeaderValue) -> Result<String, CentraleError> {
    if let Ok(subdomain) = value.to_str() {
        if subdomain.is_empty() {
            Err(CentraleError::MissingSubdomain)
        } else {
            Ok(subdomain.to_string())
        }
    } else {
        Err(CentraleError::InvalidSubdomain)
    }
}

#[actix_rt::test]
async fn empty_subdomain_error() {
    use crate::request::handle_request;

    //
    use actix_web::{App, test, web};
    let app = test::init_service(App::new().route("/", web::get().to(handle_request))).await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("Host", ""))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}
