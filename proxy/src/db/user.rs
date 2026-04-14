use crate::error::CentraleError;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
//
pub fn create_user_table(
    db: &PooledConnection<SqliteConnectionManager>,
) -> Result<(), CentraleError> {
    db.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS user (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE CHECK(
                LENGTH(username) >= 1
                AND LENGTH(username) <= 100
                AND username NOT GLOB '*[^a-zA-Z0-9-]*'   -- only alphanumeric + hyphens
            ),
            password TEXT NOT NULL CHECK(LENGTH(password) >= 1 AND LENGTH(password) <= 100),
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
