use config::CentraleConfig;
use dir_and_db_pool::db::{DbBool, DbConnection};

use crate::error::CentraleError;

pub fn get_encrypted_connection(pool: &DbBool) -> Result<DbConnection, CentraleError> {
    let db = pool.get()?;
    db.execute_batch(&format!(
        "PRAGMA key = '{}';",
        CentraleConfig::MASTER_PASSWORD
    ))?;

    Ok(db)
}
