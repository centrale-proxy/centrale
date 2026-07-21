mod api;
mod db;
mod error;
mod handle_payload;
mod parse_checkin;
mod poll;
mod routes;
mod server;
mod server_actix;
mod server_mio;
mod subdomain;

use crate::server::start_server;
use config::CentraleConfig;
use dir_and_db_pool::db::get_db::get_db;

use dotenvy::dotenv;

fn main() {
    dotenv().ok();
    let db = get_db(CentraleConfig::WRITER_DB_FILE, CentraleConfig::DB_FOLDER).unwrap();
    let bytes_db = get_db(&"bytes.db".to_string(), CentraleConfig::DB_FOLDER).unwrap();
    match start_server(db, bytes_db) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("err, {}", err)
        }
    }
}
