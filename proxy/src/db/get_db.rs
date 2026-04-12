use crate::error::CentraleError;
use config::CentraleConfig;
use dir_and_db_pool::db::{
    DbBool, DbConnection, get_encrypted_connection::get_encrypted_connection,
};

use log::error;
use std::env;

pub fn get_centrale_db(pool: &DbBool) -> Result<DbConnection, CentraleError> {
    let password = match env::var(CentraleConfig::CENTRALE_MASTER_PASSWORD) {
        Ok(token) => token,
        Err(err) => {
            error!("{}", err);
            return Err(CentraleError::MissingMasterPassword);
        }
    };
    let db = get_encrypted_connection(pool, &password)?;
    Ok(db)
}
