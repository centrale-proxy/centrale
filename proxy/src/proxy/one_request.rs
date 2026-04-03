use crate::{
    error::CentraleError,
    proxy::{get_user_id::get_user_id, host::get_host, subdomain::get_subdomain},
    subdomain::{get::get_subdomain_pass, get_subdomain_user::get_subdomain_user_role},
};
use actix_http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, web};
use config::CentraleConfig;
use dir_and_db_pool::db::DbBool;
use reqwest::header;

/// Process one wildcard request
pub async fn process_one_request(
    pool: web::Data<DbBool>,
    req: HttpRequest,
) -> Result<HttpResponse, CentraleError> {
    let headers = req.headers();
    let host = get_host(headers)?;
    let subdomain = get_subdomain(host)?;
    // AUTHENTICATE
    let user_id = get_user_id(pool.clone(), headers, req.cookie("centrale"))?;
    // AUTHORIZE
    let subdomain_user_role = get_subdomain_user_role(&pool, &subdomain, user_id)?;
    // GET PASS
    let pass = get_subdomain_pass(&pool, &subdomain)?;
    // PREPARE TO PROXY
    let path = req.path().to_string();
    let domain = format!("http://localhost:8000");
    let url = format!("{}{}", domain, path);
    // PROXY
    let client = reqwest::Client::new();
    let master_token = CentraleConfig::MASTER_BEARER_TOKEN;
    let response = client
        .get(&url)
        .header(header::AUTHORIZATION, format!("Bearer {}", master_token))
        .header("centrale_subdomain", format!("{}", subdomain))
        .header("centrale_password", format!("{}", pass))
        .header("centrale_role", format!("{}", subdomain_user_role))
        .send()
        .await?;

    let status = response.status();
    let body = response.bytes().await?;
    let res = HttpResponse::build(StatusCode::from_u16(status.as_u16()).unwrap()).body(body);

    Ok(res)
}
