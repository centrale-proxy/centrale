use crate::db::{init_bytes_db, init_writer_db};
use crate::server_actix::start_server_actix;
use crate::server_mio::start_server_mio;
use dir_and_db_pool::db::DbPool;
use std::error::Error;

pub fn start_server(pool: DbPool, bytes_pool: DbPool) -> Result<(), Box<dyn Error>> {
    let (feed_tx, _) = tokio::sync::broadcast::channel(256);

    // INIT WRITER DB
    let db = pool.get()?;
    init_writer_db(&db)?;
    let bytes_db = bytes_pool.get()?;
    init_bytes_db(&bytes_db)?;

    let actix_feed_tx = feed_tx.clone();
    let p = pool.clone();
    let actix_bytes_pool = bytes_pool.clone();
    std::thread::spawn(move || {
        if let Err(err) = actix_web::rt::System::new().block_on(start_server_actix(
            actix_feed_tx,
            p,
            actix_bytes_pool,
        )) {
            eprintln!("Actix server error: {err}");
        }
    });

    start_server_mio(pool.clone(), bytes_pool, feed_tx)
}
