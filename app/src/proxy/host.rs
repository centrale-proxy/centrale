use crate::error::CentraleError;
use actix_web::http::header::{HeaderMap, HeaderValue};

pub fn get_host(headaer: &HeaderMap) -> Result<&HeaderValue, CentraleError> {
    let host = headaer.get("Host");
    let referer = headaer.get("Referer");
    if host.is_some() {
        Ok(host.unwrap())
    } else if referer.is_some() {
        Ok(referer.unwrap())
    } else {
        Err(CentraleError::NoHostNoReferer)
    }
}
