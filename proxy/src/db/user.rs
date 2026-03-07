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
        )
        ",
    )?;

    create_bearer_table(db)?;

    create_cookie_table(db)?;

    Ok(())
}

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

        CREATE INDEX IF NOT EXISTS idx_bearer ON bearer (bearer, user_id);

        ",
    )?;
    Ok(())
}

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

        CREATE INDEX IF NOT EXISTS idx_cookie ON cookie (cookie, user_id);

        ",
    )?;
    Ok(())
}
