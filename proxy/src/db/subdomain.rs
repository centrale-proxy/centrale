use crate::error::CentraleError;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

pub fn create_subdomain_table(
    db: &PooledConnection<SqliteConnectionManager>,
) -> Result<(), CentraleError> {
    // TBD address
    // TBD ip
    // TBD port
    // TBD allowed_origin
    db.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS subdomain (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            subdomain TEXT NOT NULL UNIQUE CHECK(
                LENGTH(subdomain) >= 1
                AND LENGTH(subdomain) <= 20
                AND subdomain NOT GLOB '*[^a-zA-Z0-9-]*'   -- only alphanumeric + hyphens
            ),
            password TEXT NOT NULL CHECK(password <> ''),
            user_id INTEGER NOT NULL,
            FOREIGN KEY(user_id) REFERENCES user(id)
        );

        CREATE INDEX IF NOT EXISTS idx_subdomain ON subdomain (subdomain, user_id);
        ",
    )?;

    Ok(())
}
