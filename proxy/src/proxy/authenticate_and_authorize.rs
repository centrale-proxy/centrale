use crate::{
    error::CentraleError,
    proxy::{get_user_id::get_user_id, host::get_host, subdomain::get_subdomain},
    subdomain::{get::get_subdomain_pass, get_subdomain_user::get_subdomain_user_role},
};
use actix_web::{HttpRequest, web};
use dir_and_db_pool::db::DbBool;

pub fn authenticate_and_authorize(
    pool: web::Data<DbBool>,
    req: HttpRequest,
) -> Result<(i64, String, String, String, String), CentraleError> {
    let headers = req.headers();
    let host = get_host(headers)?;
    // VALIDATE SUBDOMAIN
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

    Ok((user_id, subdomain, subdomain_user_role, pass, url))
}
