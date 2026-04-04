use crate::error::CentraleError;
use actix_web::HttpRequest;

pub fn get_host(req: &HttpRequest) -> Result<String, CentraleError> {
    let header = req.headers();
    let host = header.get("Host");
    let referer = header.get("Referer");
    let from_url = req.uri();
    if host.is_some() {
        Ok(host.unwrap().to_str().unwrap().to_string())
    } else if referer.is_some() {
        Ok(referer.unwrap().to_str().unwrap().to_string())
    } else {
        Ok(from_url.to_string())
        // Err(CentraleError::NoHostNoReferer)
    }
}
