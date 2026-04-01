use config::CentraleConfig;
use dir_and_db_pool::db::get_db::get_db;

fn main() {
    let _db = get_db(CentraleConfig::DB_FILE, CentraleConfig::DB_FOLDER).unwrap();
    /*
    match start_server(db) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("err, {}", err)
        }
    }
     */
}
