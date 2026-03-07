use crate::{
    error::CentraleError,
    proxy::{get_user_id::get_user_id, host::get_host, subdomain::get_subdomain},
};
use actix_web::{HttpRequest, HttpResponse, web};
use dir_and_db_pool::db::DbBool;

/// Process one wildcard request
pub fn process_one_request(
    pool: web::Data<DbBool>,
    req: HttpRequest,
) -> Result<HttpResponse, CentraleError> {
    let headers = req.headers();
    let host = get_host(headers)?;
    println!(" host: {:?}", &host);
    let subdomain = get_subdomain(host)?;
    println!(" subdomain: {:?}", &subdomain);
    let user_id = get_user_id(pool, headers, req.cookie("centrale"))?;
    println!(" user_id: {:?}", &user_id);
    let res = HttpResponse::Ok().json(serde_json::json!({ "Ok": subdomain, "user": user_id }));
    Ok(res)
}
