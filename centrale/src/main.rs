mod config;
mod request;
mod server;

use crate::{config::CentraleConfig, server::start_server};
use dir_and_db_pool::db::get_db::get_db;

fn main() {
    env_logger::init();
    let db = get_db(CentraleConfig::DB_FILE, CentraleConfig::DB_FOLDER).unwrap();
    start_server(db);
}
