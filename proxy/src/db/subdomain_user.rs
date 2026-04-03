use crate::error::CentraleError;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

pub fn create_subdomain_user_table(
    db: &PooledConnection<SqliteConnectionManager>,
) -> Result<(), CentraleError> {
    db.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS subdomain_user (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            subdomain TEXT NOT NULL UNIQUE CHECK(subdomain <> ''),
            user_id INTEGER NOT NULL,
            role TEXT NOT NULL,
            FOREIGN KEY(user_id) REFERENCES user(id)
            FOREIGN KEY(subdomain) REFERENCES subdomain(subdomain)
        );

        CREATE INDEX IF NOT EXISTS idx_subdomain_user ON subdomain_user (subdomain, user_id);
        ",
    )?;

    Ok(())
}
