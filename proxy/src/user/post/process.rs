use crate::{
    db::get_db::get_encrypted_connection,
    error::CentraleError,
    user::{
        cookie::save_cookie::save_cookie,
        post::{
            add_to_db::add_user_to_db, cookie::create_and_set_cookie, hash_and_salt::hash_and_salt,
        },
    },
};
use actix_web::{HttpResponse, web};
use dir_and_db_pool::db::DbBool;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub password: String,
}

/// Main worker for user posting
pub fn handle_register(
    pool: web::Data<DbBool>,
    json: web::Json<RegisterUser>,
) -> Result<HttpResponse, CentraleError> {
    let register_request = json.into_inner();
    let username = register_request.username;
    let password = register_request.password;
    let db = get_encrypted_connection(pool.get_ref())?;
    // CREATE HASH AND SALT
    let (hash, salt) = hash_and_salt(&password)?;
    // SAVE USER TO DB
    let user_id = add_user_to_db(&db, &username, &hash, salt.as_str())?;
    // SAVE COOKIE
    let cookie_value = save_cookie(&db, user_id)?;
    // ADD COOKIE
    let resp = create_and_set_cookie(cookie_value, user_id)?;
    Ok(resp)
}
