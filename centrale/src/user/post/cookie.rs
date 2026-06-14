use crate::error::CentraleError;
use actix_web::{
    HttpResponse,
    cookie::{Cookie, SameSite, time::Duration},
};
use config::CentraleConfig;

/// Create cookie and add it to response
pub fn create_and_set_cookie(
    cookie_value: String,
    user_id: i64,
) -> Result<HttpResponse, CentraleError> {
    // DO COOKIE
    let cookie = Cookie::build("centrale", cookie_value)
        .domain(CentraleConfig::get("DOMAIN"))
        .max_age(Duration::new(
            CentraleConfig::get("COOKIE_TIMEOUT").parse::<i64>()?,
            0,
        ))
        .secure(CentraleConfig::get("COOKIE_SECURE").parse::<bool>()?)
        .http_only(CentraleConfig::get("COOKIE_HTTP_ONLY").parse::<bool>()?)
        .same_site(SameSite::Lax)
        // Set-Cookie: centrale=...; Domain=localhost.com; Max-Age=86400; SameSite=None; Secure
        .path("/")
        .finish();
    // ADD COOKIE
    let resp = HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({ "user_id": user_id.to_string() }));

    Ok(resp)
}
