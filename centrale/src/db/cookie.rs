use crate::error::CentraleError;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

pub fn create_cookie_table(
    db: &PooledConnection<SqliteConnectionManager>,
) -> Result<(), CentraleError> {
    db.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS cookie (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            cookie TEXT,
            user_id INTEGER,
            timeout INTEGER,
            FOREIGN KEY(user_id) REFERENCES user(id)
        );

        CREATE INDEX IF NOT EXISTS idx_cookie ON cookie (cookie);

        ",
    )?;
    Ok(())
}
