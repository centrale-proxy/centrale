mod connect_to_port;
mod convert;
mod error;
mod one_connection;
mod one_message;
mod payload;
mod poll;
mod save_to_db;
mod server;

use crate::server::start_server;
use config::CentraleConfig;
use dir_and_db_pool::db::get_db::get_db;

fn main() {
    let db = get_db(CentraleConfig::DB_FILE, CentraleConfig::DB_FOLDER).unwrap();
    start_server(db).unwrap();
}
