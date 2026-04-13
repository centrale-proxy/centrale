mod db;
mod error;
mod proxy;
mod server;
mod subdomain;
mod user;

use crate::server::setup_and_start;
use config::CentraleConfig;
use dotenvy::dotenv;
use log::error;

fn main() {
    // START LOBBING
    env_logger::init();
    // DO NOT PANIC, IF .env IS MISSING
    dotenv().ok();
    // TEST IF ALL VARS ARE THERE
    CentraleConfig::test();
    if let Err(err) = setup_and_start() {
        error!("Centrale error: {}", err);
    }
}
