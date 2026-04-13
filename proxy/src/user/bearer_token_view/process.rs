use crate::{
    error::CentraleError, proxy::get_user_id::get_user_id,
    user::bearer_token_view::find_bearer_tokens::find_bearer_tokens,
};
use actix_web::{HttpRequest, HttpResponse, web};
use dir_and_db_pool::db::DbBool;
//
pub fn process_view_bearer_tokens(
    pool: web::Data<DbBool>,
    req: HttpRequest,
) -> Result<HttpResponse, CentraleError> {
    let headers = req.headers();
    let user_id = get_user_id(pool.clone(), headers, req.cookie("centrale"))?;
    // view
    let tokens = find_bearer_tokens(&pool, user_id)?;
    // generate
    let resp = HttpResponse::Ok()
        .json(serde_json::json!({ "user_id": user_id.to_string(), "tokens": tokens }));
    Ok(resp)
}
