use actix_web::dev::ServiceRequest;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckIn {
    pub checkin: u128,
    pub ip: ClientIP,
    pub bytes: Vec<u8>,
    pub host: Option<String>,
    pub x_id: String,
}

impl CheckIn {
    pub fn new(ip: ClientIP, bytes: Vec<u8>, x_id: String, host: Option<String>) -> Self {
        // GET TIME
        let epoch_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap() // TBD
            .as_millis();

        CheckIn {
            checkin: epoch_time,
            ip,
            bytes,
            host,
            x_id,
        }
    }
}

use crate::client_ip::ClientIP;
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
pub struct CheckOut {
    pub checkout: u128,
    pub error: Option<String>,
    pub status: Option<u16>,
    pub x_id: String,
}

impl CheckOut {
    pub fn new(status: Option<u16>, error: Option<String>, x_id: String) -> Self {
        let epoch_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        CheckOut {
            checkout: epoch_time,
            error: error,
            status: status,
            x_id: x_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CentralePingInput {
    pub counter: u16,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CentralePing {
    pub counter: u16,
    pub url: String,
    pub ip: String,
    pub host: Option<String>,
}

impl CentralePing {
    pub fn new(counter: u16, url: &String, ip: String, host: Option<String>) -> Self {
        CentralePing {
            counter: counter,
            url: url.to_string(),
            ip: ip,
            host: host,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WriterPayload {
    CheckIn(CheckIn),
    CheckOut(CheckOut),
    CentralePing(CentralePing),
}
