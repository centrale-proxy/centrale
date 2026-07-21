use crate::{
    api::subdomain::post::{
        get_pass_and_address::get_subdomain_data, get_subdomain_user::get_subdomain_user_role,
    },
    error::CentraleError,
    proxy::auth::{get_user_id::get_user_id, subdomain::get_subdomain},
};
use actix_web::{HttpRequest, web};
use dir_and_db_pool::db::DbPool;
///
pub fn authenticate_and_authorize(
    pool: web::Data<DbPool>,
    req: &HttpRequest,
    host: &str,
) -> Result<(i64, String, String, String, String, String, String, bool), CentraleError> {
    // println!("req {:?}", &req);
    let headers = req.headers();
    // VALIDATE SUBDOMAIN
    let subdomain = get_subdomain(&host)?;
    // AUTHENTICATE
    let user_id = get_user_id(pool.clone(), headers, req.cookie("centrale"))?;
    // AUTHORIZE
    let subdomain_user_role = get_subdomain_user_role(&pool, &subdomain, user_id)?;
    // GET PASS AND ADDRESS
    //
    let subdomain_data = get_subdomain_data(&pool, &subdomain)?;
    // PREPARE TO PROXY
    let path = req.path().to_string();
    let domain = format!("{}", subdomain_data.address);
    let url = format!("{}{}", domain, path);

    Ok((
        user_id,
        subdomain,
        subdomain_user_role,
        subdomain_data.password,
        url,
        subdomain_data.destination_bearer,
        subdomain_data.name,
        subdomain_data.serve_front,
    ))
}
