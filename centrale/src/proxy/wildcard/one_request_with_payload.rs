use crate::{error::CentraleError, server::auth::CentraleUser};
use actix_http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, web};
use reqwest::{Method, header};
use std::str::FromStr;

/// Process one wildcard request
pub async fn process_one_request_with_payload(
    req: HttpRequest,
    body: web::Bytes,
    client: web::Data<reqwest::Client>,
    user: CentraleUser,
) -> Result<HttpResponse, CentraleError> {
    let method =
        Method::from_str(req.method().as_str()).map_err(|_| CentraleError::InvalidMethod)?;

    let https = format!("https://{}", user.url);
    let mut request = client
        .request(method.clone(), https)
        .header(
            header::AUTHORIZATION,
            format!("Bearer {}", user.destination_bearer),
        )
        .header("centrale_subdomain", user.subdomain.to_string())
        .header("centrale_password", user.pass.to_string())
        .header("centrale_role", user.role.to_string());

    // Forward the original Content-Type as-is (multipart boundary included)
    if let Some(ct) = req.headers().get(actix_web::http::header::CONTENT_TYPE) {
        if let Ok(ct_str) = ct.to_str() {
            request = request.header(header::CONTENT_TYPE, ct_str);
        }
    }

    // Forward the body byte-for-byte — no JSON round-trip
    if !body.is_empty() {
        request = request.body(body.to_vec());
    }

    let response = request.send().await?;
    let status = response.status();

    // Preserve upstream's Content-Type on the way back too
    let upstream_ct = response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_owned());

    let body = response.bytes().await?;

    let mut builder = HttpResponse::build(StatusCode::from_u16(status.as_u16()).unwrap());
    if let Some(ct) = upstream_ct {
        builder.insert_header((actix_web::http::header::CONTENT_TYPE, ct));
    }
    Ok(builder.body(body))
}
