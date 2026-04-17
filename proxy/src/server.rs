pub mod auth;
pub mod auth_2;
// pub mod auth_wrap;
pub mod log;
pub mod public_rate_limiter;
pub mod rate_limiter;
pub mod routes;
pub mod start;

use crate::{db::init::init_db, error::CentraleError, server::start::start_server};
use config::CentraleConfig;
use dir_and_db_pool::db::{db_file::db_file, encrypted::get_secret_db};

pub fn setup_and_start() -> Result<(), CentraleError> {
    let file_path = db_file(CentraleConfig::DB_FILE, CentraleConfig::DB_FOLDER).unwrap();
    let path = file_path.to_str().unwrap();
    let password = CentraleConfig::master_password();
    let db = get_secret_db(path, &password)?;
    init_db(&db)?;

    /*
       // INIT POOL REQUESTS
       let pools = HashMap::new();
       let registry = Arc::new(RwLock::new(DbPoolRegistry { pools }));
    */
    start_server(db)?;
    Ok(())
}
/*
pub struct DbPoolRegistry {
    pub pools: HashMap<String, DbBool>,
}

impl DbPoolRegistry {
    pub fn get(&self, key: &str) -> Option<&DbBool> {
        self.pools.get(key)
    }
}
 */
