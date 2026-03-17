mod connect_to_port;
mod convert;
mod error;
mod listen;
mod one_connection;
mod one_message;
mod payload;
mod poll;
mod save_to_db;

use config::CentraleConfig;
use dir_and_db_pool::db::get_db::get_db;

use crate::listen::listen_to_port;

// GET DB
// START SERVER
// OPEN CHANNEL
// RECEIVE MESSAGE

fn main() {
    let db = get_db(CentraleConfig::DB_FILE, CentraleConfig::DB_FOLDER).unwrap();
    match listen_to_port(db) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Writer Err: {}", err);
        }
    }
}
