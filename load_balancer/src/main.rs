mod connect_to_writer;
mod error;
mod load_balancer;
mod request;
mod response;
mod start;

use crate::start::start;
use dotenvy::dotenv;
use log::error;

fn main() {
    // SETUP
    env_logger::init();
    dotenv().ok();

    match start() {
        Ok(_) => {}
        Err(err) => {
            error!("Load balancer error: {:?}", err)
        }
    }
}
