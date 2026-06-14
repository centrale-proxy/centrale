use crate::{
    error::CentraleError, server::auth::CentraleUser, user::bearer_token::save::save_bearer_token,
};
use actix_web::{HttpResponse, web};
use dir_and_db_pool::db::DbPool;

pub fn process_generate_bearer_token(
    pool: web::Data<DbPool>,
    user_id_url: web::Path<i64>,
    user: CentraleUser,
) -> Result<HttpResponse, CentraleError> {
    // MAKE SURE USER MATCHES THE URL USER
    if user.user_id != *user_id_url {
        return Err(CentraleError::WrongUser);
    }
    let db = pool.get()?;
    let token = save_bearer_token(&db, user.user_id)?;
    // generate
    let resp = HttpResponse::Ok()
        .json(serde_json::json!({ "user_id": user.user_id.to_string(), "token": token }));
    Ok(resp)
}
