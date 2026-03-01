mod config;
mod db;
mod error;
mod request;
mod server;

use crate::{config::CentraleConfig, db::db_error::serve_db_error, server::start_server};
use dir_and_db_pool::db::get_db::get_db;
use log::error;

fn main() {
    env_logger::init();
    match get_db(CentraleConfig::DB_FILE, CentraleConfig::DB_FOLDER) {
        Ok(db) => {
            start_server(db);
        }
        Err(err) => {
            error!("DB error: {}", err);
            serve_db_error();
        }
    }
}
