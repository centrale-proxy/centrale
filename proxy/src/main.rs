// mod _config;
mod db;
mod error;
mod proxy;
mod server;
mod subdomain;
mod user;

use crate::{
    db::{db_error::serve_db_error, init::init_db},
    server::start::start_server,
};
use config::CentraleConfig;
use dir_and_db_pool::db::{db_file::db_file, encrypted::get_secret_db, get_db::get_db};
use log::error;

fn main() {
    // std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let file_path = db_file(CentraleConfig::DB_FILE, CentraleConfig::DB_FOLDER).unwrap();
    let path = file_path.to_str().unwrap();
    match get_secret_db(path, CentraleConfig::MASTER_PASSWORD) {
        Ok(db) => match init_db(&db) {
            Ok(_) => match start_server(db) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("server error {:?}", err);
                }
            },
            Err(err) => {
                eprintln!("db error {:?}", err);
            }
        },
        Err(err) => {
            error!("DB error: {}", err);
            serve_db_error();
        }
    }
}
