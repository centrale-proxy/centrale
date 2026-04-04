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
        .domain(CentraleConfig::DOMAIN)
        .max_age(Duration::new(CentraleConfig::COOKIE_TIMEOUT, 0))
        .secure(CentraleConfig::COOKIE_SECURE) // Only send over HTTPS
        .http_only(CentraleConfig::COOKIE_HTTP_ONLY) // Not accessible via JavaScript
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
