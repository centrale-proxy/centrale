use crate::{
    db::get_db::get_centrale_db,
    error::CentraleError,
    user::{
        cookie::{create::create_cookie, save_cookie::save_cookie},
        login::{
            find_user_by_hash::find_user_by_hash, find_user_salt::find_user_salt,
            hash_with_salt::hash_with_salt,
        },
    },
};
use actix_web::{HttpResponse, web};
use dir_and_db_pool::db::DbBool;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

/// Main worker for user posting
pub fn process_login(
    pool: web::Data<DbBool>,
    json: web::Json<LoginUser>,
) -> Result<HttpResponse, CentraleError> {
    let register_request = json.into_inner();
    let username = register_request.username;
    let password = register_request.password;
    let db = get_centrale_db(pool.get_ref())?;

    let salt = find_user_salt(&pool, &username)?;
    // CREATE HASH AND SALT
    let hash = hash_with_salt(&password, &salt)?;
    // SAVE USER TO DB
    let user_id = find_user_by_hash(&pool, &hash)?;
    // let user_id = add_user_to_db(&db, &username, &hash, salt.as_str())?;
    // SAVE COOKIE
    let cookie_value = save_cookie(&db, user_id)?;
    // ADD COOKIE
    let cookie = create_cookie(cookie_value)?;

    let resp = HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({ "user_id": user_id.to_string() }));

    Ok(resp)
}
