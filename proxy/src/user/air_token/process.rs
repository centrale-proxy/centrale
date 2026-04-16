use crate::{
    error::CentraleError, proxy::auth::get_user_id::get_user_id,
    user::air_token::save::save_air_token,
};
use actix_web::{HttpRequest, HttpResponse, web};
use dir_and_db_pool::db::DbBool;
//
pub fn process_air_token(
    pool: web::Data<DbBool>,
    req: HttpRequest,
) -> Result<HttpResponse, CentraleError> {
    let headers = req.headers();
    let user_id = get_user_id(pool.clone(), headers, req.cookie("centrale"))?;
    let db = pool.get()?;
    let token = save_air_token(&db, user_id)?;
    // generate
    let resp = HttpResponse::Ok()
        .json(serde_json::json!({ "user_id": user_id.to_string(), "token": token }));
    Ok(resp)
}
