use crate::error::CentraleError;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

pub fn create_user_table(
    db: &PooledConnection<SqliteConnectionManager>,
) -> Result<(), CentraleError> {
    db.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS user (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE CHECK(username <> ''),
            password TEXT NOT NULL CHECK(password <> ''),
            salt TEXT NOT NULL CHECK(salt <> ''),
            name TEXT,
            first_name TEXT,
            last_name TEXT,
            personal_code TEXT,
            email TEXT
        );

        CREATE INDEX IF NOT EXISTS index_user_username ON user (username);

        ",
    )?;

    Ok(())
}
