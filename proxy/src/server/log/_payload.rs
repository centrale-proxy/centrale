use std::net::IpAddr;

use actix_web::{HttpRequest, dev::ServiceRequest};
use serde_derive::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckIn {
    checkin: u128,
    ip: Option<String>,
    url: Option<String>,
    // path: Option<String>,
    query: Option<String>,
    ua: Option<String>,
    method: Option<String>,
    // referrer: Option<String>,
    // host: Option<String>,
    // os: Option<String>,
    // browser: Option<String>,
    // is_bot: bool,
    // is_admin: bool,
    //
    // body: Option<String>,
    // lead: Option<String>,
    // campaign: Option<String>,
}

use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_ip(req: &ServiceRequest) -> Option<String> {
    req.headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .or_else(|| {
            req.connection_info()
                .realip_remote_addr()
                .map(|s| s.to_string())
        })
}

pub fn get_ua(req: &ServiceRequest) -> Option<String> {
    let a = req.headers().get("user-agent");
    match a {
        Some(a) => {
            let b = a.to_str();
            match b {
                Ok(b) => Some(b.to_string()),
                Err(_err) => None,
            }
        }
        None => None,
    }
}

impl CheckIn {
    pub fn new(req: &ServiceRequest) -> Self {
        // println!("req {:?}", &req);

        // GET TIME
        let epoch_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap() // TBD
            .as_millis();

        let ip = get_ip(req);

        let query = req.query_string().to_string();

        //  let url = Url::parse(&req.uri().to_string());
        let url = req.uri().to_string();

        let ua = get_ua(&req);

        CheckIn {
            checkin: epoch_time,
            ip: ip,
            url: Some(req.uri().to_string()),
            // path: (),
            query: Some(query),
            ua: ua,
            method: Some(req.method().to_string()),
            // referrer: Some(req.method().to_string()),
            // host: (),
            // os: (),
            // browser: (),
            // is_bot: (),
            // is_admin: (),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckOut {}

#[derive(Serialize, Deserialize, Debug)]
pub enum WriterPayload {
    CheckIn(CheckIn),
    CheckOut(CheckOut),
}
