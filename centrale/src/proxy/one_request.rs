use crate::{
    error::CentraleError,
    proxy::{host::get_host, subdomain::get_subdomain},
};
use actix_web::{HttpRequest, HttpResponse};

/// Process one wildcard request
pub fn process_one_request(req: HttpRequest) -> Result<HttpResponse, CentraleError> {
    let headers = req.headers();
    let host = get_host(headers)?;
    let subdomain = get_subdomain(host)?;
    let res = HttpResponse::Ok().json(serde_json::json!({ "Ok": subdomain }));
    Ok(res)
}
