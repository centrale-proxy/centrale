use crate::{
    error::CentraleError,
    proxy::{get_user_id::get_user_id, host::get_host, subdomain::get_subdomain},
};
use actix_web::{HttpRequest, HttpResponse};

/// Process one wildcard request
pub fn process_one_request(req: HttpRequest) -> Result<HttpResponse, CentraleError> {
    let headers = req.headers();
    let host = get_host(headers)?;
    let subdomain = get_subdomain(host)?;
    let user_id = get_user_id(headers, req.cookie("centrale"))?;

    let res = HttpResponse::Ok().json(serde_json::json!({ "Ok": subdomain, "user": user_id }));
    Ok(res)
}
