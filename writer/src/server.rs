use crate::db::{init_bytes_db, init_writer_db};
use crate::server_actix::start_server_actix;
use crate::server_mio::start_server_mio;
use dir_and_db_pool::db::DbPool;
use std::error::Error;

pub fn start_server(pool: DbPool, bytes_pool: DbPool) -> Result<(), Box<dyn Error>> {
    // INIT WRITER DB
    let db = pool.get()?;
    init_writer_db(&db)?;
    let bytes_db = bytes_pool.get()?;
    init_bytes_db(&bytes_db)?;

    std::thread::spawn(|| {
        if let Err(err) = actix_web::rt::System::new().block_on(start_server_actix()) {
            eprintln!("Actix server error: {err}");
        }
    });

    start_server_mio(pool, bytes_pool)
}
