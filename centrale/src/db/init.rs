use crate::{db::user::create_user_table, error::CentraleError};
use dir_and_db_pool::db::DbBool;

pub fn init_db(pool: &DbBool) -> Result<(), CentraleError> {
    // USER TABLE
    let db = pool.get()?;
    create_user_table(&db)?;
    Ok(())
}
