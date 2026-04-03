use crate::error::CentraleError;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

pub fn create_air_token_table(
    db: &PooledConnection<SqliteConnectionManager>,
) -> Result<(), CentraleError> {
    db.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS air_token (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            air_token TEXT,
            user_id INTEGER,
            timeout INTEGER,
            FOREIGN KEY(user_id) REFERENCES user(id)
        );

        CREATE INDEX IF NOT EXISTS air_token ON air_token (air_token);

        ",
    )?;
    Ok(())
}
