use crate::{db::get_subdomain_db, error::SampleServerError, server::DbPoolRegistry};
use dir_and_db_pool::db::DbBool;
use std::sync::{Arc, RwLock};

pub fn get_or_create_from_registry(
    registry: &Arc<RwLock<DbPoolRegistry>>,
    subdomain: &str,
    pass: &str,
) -> Result<DbBool, SampleServerError> {
    // First try with just a read lock
    {
        let reg = registry
            .read()
            .map_err(|_| SampleServerError::StringError("Lock error".to_string()))?;
        if let Some(pool) = reg.pools.get(subdomain) {
            return Ok(pool.clone());
        }
    } // read lock drops here

    // Not found — acquire write lock and insert
    let new_pool = get_subdomain_db(subdomain, pass)?;
    let mut reg = registry
        .write()
        .map_err(|_| SampleServerError::StringError("Lock error".to_string()))?;

    // Check again after acquiring write lock (another thread may have inserted in the meantime)
    Ok(reg
        .pools
        .entry(subdomain.to_string())
        .or_insert(new_pool)
        .clone())
}
