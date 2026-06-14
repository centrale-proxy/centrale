use crate::error::CentraleError;
use actix_web::cookie::{Cookie, SameSite, time::Duration};
use config::CentraleConfig;

/// Create cookie
pub fn create_cookie(cookie_value: String) -> Result<Cookie<'static>, CentraleError> {
    let cookie = Cookie::build("centrale", cookie_value)
        .domain(CentraleConfig::get("DOMAIN"))
        .max_age(Duration::new(
            CentraleConfig::get("COOKIE_TIMEOUT").parse::<i64>()?,
            0,
        ))
        .secure(CentraleConfig::get("COOKIE_SECURE").parse::<bool>()?)
        .http_only(CentraleConfig::get("COOKIE_HTTP_ONLY").parse::<bool>()?)
        .same_site(SameSite::Lax)
        .path("/")
        .finish();

    Ok(cookie.into_owned())
}
