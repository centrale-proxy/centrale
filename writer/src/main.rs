mod db;
mod error;
mod handle_payload;
mod parse_checkin;
mod poll;
mod server;
mod subdomain;

use crate::server::start_server;
use config::CentraleConfig;
use dir_and_db_pool::db::get_db::get_db;

use dotenvy::dotenv;

fn main() {
    dotenv().ok();

    let db = get_db(CentraleConfig::WRITER_DB_FILE, CentraleConfig::DB_FOLDER).unwrap();
    match start_server(db) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("err, {}", err)
        }
    }
}
