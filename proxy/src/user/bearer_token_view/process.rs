use crate::{
    error::CentraleError, server::auth::CentraleUser,
    user::bearer_token_view::find_bearer_tokens::find_bearer_tokens,
};
use actix_web::{HttpResponse, web};
use dir_and_db_pool::db::DbBool;
//
pub fn process_view_bearer_tokens(
    pool: web::Data<DbBool>,
    user_id_url: web::Path<i64>,
    user: CentraleUser,
) -> Result<HttpResponse, CentraleError> {
    if user.user_id != *user_id_url {
        return Err(CentraleError::WrongUser);
    }
    // view
    let tokens = find_bearer_tokens(&pool, user.user_id)?;
    // generate
    let resp = HttpResponse::Ok()
        .json(serde_json::json!({ "user_id": user.user_id.to_string(), "tokens": tokens }));
    Ok(resp)
}
