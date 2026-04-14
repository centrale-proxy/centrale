use crate::error::CentraleError;
use actix_web::cookie::{Cookie, SameSite, time::Duration};
use config::CentraleConfig;

/// Create cookie
pub fn create_cookie(cookie_value: String) -> Result<Cookie<'static>, CentraleError> {
    let cookie = Cookie::build("centrale", cookie_value)
        .domain(CentraleConfig::get("DOMAIN"))
        .max_age(Duration::new(CentraleConfig::COOKIE_TIMEOUT, 0))
        .secure(CentraleConfig::COOKIE_SECURE)
        .http_only(CentraleConfig::COOKIE_HTTP_ONLY)
        .same_site(SameSite::Lax)
        .path("/")
        .finish();

    Ok(cookie.into_owned())
}
