use crate::error::CentraleError;
use actix_web::http::header::HeaderValue;
use url::Url;

pub fn extract_subdomain(parts: Vec<&str>) -> Option<String> {
    // Check if there are enough parts to have a subdomain
    if parts.len() > 2 {
        // Join all parts except the last two (which are the domain and TLD)
        Some(parts[..parts.len() - 2].join("."))
    } else {
        // No subdomain present
        None
    }
}

pub fn get_subdomain(input_url: &HeaderValue) -> Result<String, CentraleError> {
    if let Ok(url) = input_url.to_str() {
        //
        let parsed_url = Url::parse(url)?;

        let host = parsed_url.host_str();
        match host {
            Some(host) => {
                let parts: Vec<&str> = host.split('.').collect();
                let subdomain = extract_subdomain(parts);
                if subdomain.is_some() {
                    if subdomain.clone().unwrap().is_empty() {
                        Err(CentraleError::MissingSubdomain)
                    } else {
                        Ok(subdomain.unwrap().to_string())
                    }
                } else {
                    Err(CentraleError::MissingSubdomain)
                }
            }
            None => Err(CentraleError::MissingHost),
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
    println!("resp {:?}", &resp);
    assert!(resp.status().is_client_error());
}

#[actix_rt::test]
async fn normal_subdomain() {
    use crate::request::handle_request;
    //
    use actix_web::{App, test, web};
    let app = test::init_service(App::new().route("/", web::get().to(handle_request))).await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("Host", "https://hello.hello.ee"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
