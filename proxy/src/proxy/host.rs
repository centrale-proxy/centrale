use crate::error::CentraleError;
use actix_web::HttpRequest;

pub fn get_host(req: &HttpRequest) -> Result<String, CentraleError> {
    let header = req.headers();
    let host = header.get("Host");
    // Cloudflare likes Referer sometimes
    // let referer = header.get("Referer");
    let from_url = req.uri();

    if let Some(host) = host {
        Ok(host.to_str()?.to_string())
        // } else if let Some(referer) = referer {
        //    Ok(referer.to_str()?.to_string())
    } else {
        Ok(from_url.to_string())
    }
}
