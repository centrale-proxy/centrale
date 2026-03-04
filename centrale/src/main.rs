mod config;
mod db;
mod error;
mod proxy;
mod request;
mod server;
mod user;

use crate::{
    config::CentraleConfig,
    db::{db_error::serve_db_error, init::init_db},
    server::start_server,
};
use dir_and_db_pool::db::get_db::get_db;
use log::error;

fn main() {
    // std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    match get_db(CentraleConfig::DB_FILE, CentraleConfig::DB_FOLDER) {
        Ok(db) => {
            init_db(&db);
            start_server(db);
        }
        Err(err) => {
            error!("DB error: {}", err);
            serve_db_error();
        }
    }
}
