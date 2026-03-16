use crate::error::CentraleError;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

pub fn create_bearer_table(
    db: &PooledConnection<SqliteConnectionManager>,
) -> Result<(), CentraleError> {
    db.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS bearer (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            bearer TEXT,
            user_id INTEGER,
            FOREIGN KEY(user_id) REFERENCES user(id)
        );

        CREATE INDEX IF NOT EXISTS idx_bearer ON bearer (bearer);

        ",
    )?;
    Ok(())
}
