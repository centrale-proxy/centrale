use crate::error::CentraleError;
use actix_web::HttpRequest;

// TBD SHOULD TAKE from_url BY DEFAULT. TEST
pub fn get_host(req: &HttpRequest) -> Result<String, CentraleError> {
    let header = req.headers();
    let host = header.get("Host");
    // Cloudflare likes Referer sometimes
    // let referer = header.get("Referer");
    let from_url = req.uri();
    if let Some(host) = host {
        let upgrade = header.get("upgrade");
        let mut host_url = host.to_str()?.to_string();
        if upgrade.is_some() && upgrade.unwrap().to_str().unwrap() == "websocket" {
            host_url = format!("wss://{}", host_url);
        }
        Ok(host_url)
        // } else if let Some(referer) = referer {
        //    Ok(referer.to_str()?.to_string())
    } else {
        Ok(from_url.to_string())
    }
}
