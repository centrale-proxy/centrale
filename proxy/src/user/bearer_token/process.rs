use crate::{
    error::CentraleError, proxy::auth::get_user_id::get_user_id,
    user::bearer_token::save::save_bearer_token,
};
use actix_web::{HttpRequest, HttpResponse, web};
use dir_and_db_pool::db::DbBool;
//
pub fn process_generate_bearer_token(
    pool: web::Data<DbBool>,
    req: HttpRequest,
    user_id_url: web::Path<i64>,
) -> Result<HttpResponse, CentraleError> {
    let headers = req.headers();
    let user_id = get_user_id(pool.clone(), headers, req.cookie("centrale"))?;
    // MAKE SURE USER MATCHES THE URL USER
    if user_id != *user_id_url {
        return Err(CentraleError::WrongUser);
    }
    let db = pool.get()?;
    let token = save_bearer_token(&db, user_id)?;
    // generate
    let resp = HttpResponse::Ok()
        .json(serde_json::json!({ "user_id": user_id.to_string(), "token": token }));
    Ok(resp)
}
