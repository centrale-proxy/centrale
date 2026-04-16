use crate::{
    error::CentraleError,
    proxy::{
        auth::{host::get_host, subdomain_string::get_subdomain_string},
        wildcard::QueryParams,
    },
    subdomain::{get::get_subdomain_pass, get_subdomain_user::get_subdomain_user_role},
    user::air_token::find_user_by_air_token::find_user_by_air_token,
};
use actix_web::{HttpRequest, web};
use config::CentraleConfig;
use dir_and_db_pool::db::DbBool;
///
pub fn ws_authenticate_and_authorize(
    pool: web::Data<DbBool>,
    req: &HttpRequest,
    query: web::Query<QueryParams>,
) -> Result<(i64, String, String, String, String), CentraleError> {
    //let headers = req.headers();
    let host = get_host(req)?;
    // VALIDATE SUBDOMAIN
    let full_url = format!("https://{}", host);

    let subdomain = get_subdomain_string(&full_url)?;

    let air_token = match &query.air_token {
        Some(air_token) => air_token,
        None => return Err(CentraleError::NoAirToken),
    };
    // AUTHENTICATE VIA AIR TOKEN
    let user_id = find_user_by_air_token(&pool, &air_token)?;
    // AUTHORIZE
    let subdomain_user_role = get_subdomain_user_role(&pool, &subdomain, user_id)?;
    // GET PASS
    let pass = get_subdomain_pass(&pool, &subdomain)?;
    // PREPARE TO PROXY
    let path = req.path().to_string();
    let domain = format!("wss://{}", CentraleConfig::get("SAMPLE_SERVER_ADDRESS"));
    let url = format!("{}{}", domain, path);

    Ok((user_id, subdomain, subdomain_user_role, pass, url))
}
