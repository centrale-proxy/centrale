use crate::db::db_file::db_file;
use crate::error::DirsqlError;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
// use std::time::Duration;

pub fn get_db(file: &str, folder: &str) -> Result<Pool<SqliteConnectionManager>, DirsqlError> {
    let file_path = db_file(file, folder)?;
    let manager = SqliteConnectionManager::file(file_path);
    //  let pool = Pool::new(manager).expect("Failed to create pool.");
    let pool = Pool::builder()
        //   .max_size(15) // Set a reasonable maximum number of connections
        //   .min_idle(Some(5)) // Maintain a minimum number of idle connections
        //   .connection_timeout(Duration::from_secs(30)) // Set a timeout for acquiring a connection
        .build(manager)
        .expect("Failed to create pool.");

    Ok(pool)
}
