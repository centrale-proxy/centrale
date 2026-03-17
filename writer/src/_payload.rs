use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckIn {
    checkin: u128,
    ip: Option<String>,
    url: Option<String>,
    // path: Option<String>,
    query: Option<String>,
    ua: Option<String>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckOut {}

#[derive(Serialize, Deserialize, Debug)]
pub enum WriterPayload {
    CheckIn(CheckIn),
    CheckOut(CheckOut),
}
