use actix_web::dev::{ServiceRequest, ServiceResponse};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckIn2 {
    pub checkin: u128,
    pub ip: Option<String>,
    pub bytes: Vec<u8>,
    pub x_id: String,
}

impl CheckIn2 {
    pub fn new(ip: Option<String>, bytes: Vec<u8>, x_id: String) -> Self {
        // GET TIME
        let epoch_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap() // TBD
            .as_millis();

        CheckIn2 {
            checkin: epoch_time,
            ip,
            bytes,
            x_id,
        }
    }
}

use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_ip(req: &ServiceRequest) -> Option<String> {
    let conn_info = req.connection_info();
    let peer_addr = conn_info.peer_addr();
    match peer_addr {
        Some(peer) => Some(peer.to_string()),
        None => None,
    }
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
//

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckOut2 {
    pub checkout: u128,
    pub error: Option<String>,
    pub status: Option<u16>,
    pub x_id: String,
}

impl CheckOut2 {
    pub fn new(status: Option<u16>, error: Option<String>, x_id: String) -> Self {
        let epoch_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        CheckOut2 {
            checkout: epoch_time,
            error: error,
            status: status,
            x_id: x_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WriterPayload {
    CheckIn2(CheckIn2),
    CheckOut2(CheckOut2),
}
