use crate::{
    error::CentraleError,
    proxy::auth::{get_user_id::get_user_id, subdomain::get_subdomain},
    subdomain::post::{
        get_pass_and_address::get_subdomain_pass_and_address,
        get_subdomain_user::get_subdomain_user_role,
    },
};
use actix_web::{HttpRequest, web};
use dir_and_db_pool::db::DbPool;
///
pub fn authenticate_and_authorize(
    pool: web::Data<DbPool>,
    req: &HttpRequest,
    host: &str,
) -> Result<(i64, String, String, String, String, String), CentraleError> {
    // println!("req {:?}", &req);
    let headers = req.headers();
    // VALIDATE SUBDOMAIN
    let subdomain = get_subdomain(&host)?;
    // AUTHENTICATE
    let user_id = get_user_id(pool.clone(), headers, req.cookie("centrale"))?;
    // AUTHORIZE
    let subdomain_user_role = get_subdomain_user_role(&pool, &subdomain, user_id)?;
    // GET PASS AND ADDRESS
    let pass_and_address = get_subdomain_pass_and_address(&pool, &subdomain)?;
    // PREPARE TO PROXY
    let path = req.path().to_string();
    let domain = format!("{}", pass_and_address.address);
    let url = format!("{}{}", domain, path);

    Ok((
        user_id,
        subdomain,
        subdomain_user_role,
        pass_and_address.password,
        pass_and_address.address,
        url,
    ))
}
