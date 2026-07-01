use crate::error::CentraleError;
use config::CentraleConfig;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

pub fn create_subdomain_table(
    db: &PooledConnection<SqliteConnectionManager>,
) -> Result<(), CentraleError> {
    let sql = format!(
        "
        CREATE TABLE IF NOT EXISTS subdomain (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            subdomain TEXT NOT NULL UNIQUE CHECK(
                LENGTH(subdomain) >= 1
                AND LENGTH(subdomain) <= {}
                AND subdomain NOT GLOB '*[^a-zA-Z0-9-]*'
            ),
            password TEXT NOT NULL CHECK(password <> ''),
            user_id INTEGER NOT NULL,
            address TEXT NOT NULL,
            name TEXT CHECK(
                LENGTH(name) >= 1
                AND LENGTH(name) <= {}
                AND name NOT GLOB '*[^a-zA-Z0-9 -]*'
            ),
            destination_bearer TEXT NOT NULL,
            FOREIGN KEY(user_id) REFERENCES user(id)
        );
        CREATE INDEX IF NOT EXISTS idx_subdomain ON subdomain (subdomain, user_id);
        ",
        CentraleConfig::MAX_SUBDOMAIN_LENGTH,
        CentraleConfig::MAX_SUBDOMAIN_NAME_LENGTH
    );

    db.execute_batch(&sql)?;
    Ok(())
}
