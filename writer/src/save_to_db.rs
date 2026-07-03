use common::payload::WriterPayload;
use dir_and_db_pool::db::DbConnection;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParsedCheckIn {
    pub checkin: u128,
    pub ip: Option<String>,
    pub url: Option<String>,
    pub query: Option<String>,
    pub ua: Option<String>, // STRING
    pub method: Option<String>,
    pub referrer: Option<String>,
    pub host: Option<String>,
    pub os: Option<String>,
    pub browser: Option<String>,  // PARSED UA
    pub is_bot: bool,             // GOOGLE OR FB CRAWLER
    pub lead: Option<String>,     // GOOGLE BING OR FB
    pub campaign: Option<String>, // utm_campaign
}

pub fn save_to_db(payload: WriterPayload, _db: &DbConnection) {
    // println!("payload: {:?}", &payload);
    match payload {
        WriterPayload::CheckIn(payload) => {
            // SEND TX OR GO STRAIGHT TO DB?
            println!(
                "{} {} {} {}",
                payload.method.unwrap_or("".to_string()),
                payload.url.unwrap_or("".to_string()),
                payload.ip.unwrap_or("".to_string()),
                payload.checkin
            );
        }
        WriterPayload::CheckIn2(payload) => {
            // SEND TX OR GO STRAIGHT TO DB?
            println!("> {}  {:?} {}", payload.x_id, payload.ip, payload.checkin);

            //let text = String::from_utf8_lossy(&payload.bytes.to_vec().clone());
            let text = String::from_utf8_lossy(&payload.bytes);

            println!("> {}", text);
        }
        WriterPayload::CheckOut(payload) => {
            println!(
                "{} {} {}",
                payload.status.unwrap_or(0),
                payload.error.unwrap_or("".to_string()),
                payload.checkout
            );
        }
        WriterPayload::CheckOut2(payload) => {
            println!(
                "< {} {} {} {}",
                payload.status.unwrap_or(0),
                payload.error.unwrap_or("".to_string()),
                payload.checkout,
                payload.x_id
            );
        }
    }
}
