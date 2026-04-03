use crate::{
    error::CentraleError,
    proxy::{
        host::get_host, subdomain::get_subdomain, subdomain_string::get_subdomain_string,
        wildcard::QueryParams,
    },
    subdomain::{get::get_subdomain_pass, get_subdomain_user::get_subdomain_user_role},
    user::air_token::find_user_by_air_token::find_user_by_air_token,
};
use actix_web::{HttpRequest, web};
use dir_and_db_pool::db::DbBool;
///
pub fn ws_authenticate_and_authorize(
    pool: web::Data<DbBool>,
    req: &HttpRequest,
    query: web::Query<QueryParams>,
) -> Result<(i64, String, String, String, String), CentraleError> {
    let headers = req.headers();
    let host = get_host(headers)?;
    // VALIDATE SUBDOMAIN
    let uuu = format!("http://{}", host.to_str()?);
    let subdomain = get_subdomain_string(&uuu)?;
    // AUTHENTICATE VIA AIR TOKEN
    let user_id = find_user_by_air_token(&pool, &query.air_token)?;
    // AUTHORIZE
    let subdomain_user_role = get_subdomain_user_role(&pool, &subdomain, user_id)?;
    // GET PASS
    let pass = get_subdomain_pass(&pool, &subdomain)?;
    // PREPARE TO PROXY
    let path = req.path().to_string();
    let domain = format!("ws://localhost:11111");
    let url = format!("{}{}", domain, path);

    Ok((user_id, subdomain, subdomain_user_role, pass, url))
}
