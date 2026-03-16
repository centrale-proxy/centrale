use actix_web::{HttpRequest, HttpResponse, web};
use dir_and_db_pool::db::DbBool;

use crate::{error::CentraleError, proxy::get_user_id::get_user_id};
//
pub fn get_user_process(
    pool: web::Data<DbBool>,
    req: HttpRequest,
) -> Result<HttpResponse, CentraleError> {
    let headers = req.headers();
    let user_id = get_user_id(pool, headers, req.cookie("centrale"))?;
    let resp = HttpResponse::Ok().json(serde_json::json!({ "user_id": user_id.to_string() }));
    Ok(resp)
}
