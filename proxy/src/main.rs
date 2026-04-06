mod db;
mod error;
mod proxy;
mod server;
mod subdomain;
mod user;

use crate::server::setup_and_start;
use log::error;

fn main() {
    env_logger::init();
    if let Err(err) = setup_and_start() {
        error!("Centrale error: {}", err);
    }
}
