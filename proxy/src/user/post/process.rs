use crate::{
    error::CentraleError,
    user::{cookie::save_cookie::save_cookie, post::add_to_db::add_user},
};
use actix_web::{
    HttpResponse,
    cookie::{Cookie, time::Duration},
    web,
};
use config::CentraleConfig;
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
    let db = pool.get().expect("Couldn't get db connection from pool");
    let user_id = add_user(&db, &username, &password)?;
    let cookie_value = save_cookie(&db, user_id)?;
    // DO COOKIE
    let cookie = Cookie::build("centrale", cookie_value)
        .domain(CentraleConfig::DOMAIN)
        .max_age(Duration::new(CentraleConfig::COOKIE_TIMEOUT, 0))
        .secure(CentraleConfig::COOKIE_SECURE) // Only send over HTTPS
        .http_only(CentraleConfig::COOKIE_HTTP_ONLY) // Not accessible via JavaScript
        // .path("/")
        .finish();
    // ADD COOKIE
    let resp = HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({ "user_id": user_id.to_string() }));

    Ok(resp)
}
