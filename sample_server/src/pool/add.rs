use crate::{error::SampleServerError, pool::DbPoolRegistry};
use dir_and_db_pool::db::DbBool;
use std::sync::{Arc, RwLock};

pub fn add_to_registry(
    registry: &Arc<RwLock<DbPoolRegistry>>,
    subdomain: String,
    pool: DbBool,
) -> Result<(), SampleServerError> {
    let mut reg = registry
        .write()
        .map_err(|_| SampleServerError::StringError("Lockerror".to_string()))?;
    reg.pools.insert(subdomain, pool);
    Ok(())
}
