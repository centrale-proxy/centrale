use crate::error::CentraleError;
use config::CentraleConfig;
use dir_and_db_pool::db::{
    DbBool, DbConnection, get_encrypted_connection::get_encrypted_connection,
};

pub fn get_centrale_db(pool: &DbBool) -> Result<DbConnection, CentraleError> {
    let password = CentraleConfig::master_password();
    let db = get_encrypted_connection(pool, &password)?;
    Ok(db)
}
