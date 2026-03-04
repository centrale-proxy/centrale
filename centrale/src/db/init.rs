use crate::{db::user::create_user_table, error::CentraleError};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub fn init_db(pool: &Pool<SqliteConnectionManager>) -> Result<(), CentraleError> {
    // USER TABLE
    let db = pool.get()?;
    create_user_table(&db)?;
    Ok(())
}
