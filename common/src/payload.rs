use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    http::StatusCode,
    web::Bytes,
};
use serde_derive::{Deserialize, Serialize};

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

use crate::convert::string_to_vector;

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
    pub fn new_vector(req: &ServiceRequest) -> Vec<u8> {
        let aaa = Self::new(req);
        let pl = WriterPayload::CheckIn(aaa);
        let sss = serde_json::to_string(&pl).unwrap();
        let uu = string_to_vector(&sss);
        uu
    }
    pub fn new(req: &ServiceRequest) -> Self {
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
            url: Some(url),
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
pub struct CheckOut {
    checkout: u128,
    error: Option<String>,
    status: Option<u16>,
}

use actix_web::body::to_bytes;

pub async fn read_response_body(res: ServiceResponse) -> String {
    let body_bytes = to_bytes(res.into_body()).await.unwrap();
    String::from_utf8(body_bytes.to_vec()).unwrap()
}

impl CheckOut {
    pub fn new(status: StatusCode, body: Option<&Bytes>) -> Self {
        let epoch_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let status_u16 = status.as_u16();
        if status_u16 != 200 {
            CheckOut {
                checkout: epoch_time,
                error: None,
                status: Some(status_u16),
            }
        } else {
            match body {
                Some(body) => {
                    let err = String::from_utf8_lossy(body).to_string();
                    CheckOut {
                        checkout: epoch_time,
                        error: Some(err),
                        status: Some(status_u16),
                    }
                }
                None => CheckOut {
                    checkout: epoch_time,
                    error: None,
                    status: Some(status_u16),
                },
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WriterPayload {
    CheckIn(CheckIn),
    CheckOut(CheckOut),
}
