use crate::error::DirsqlError;
use r2d2::{CustomizeConnection, Pool};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, Error};

#[derive(Debug)]
struct SqlCipherCustomizer {
    passphrase: String,
}

impl CustomizeConnection<Connection, Error> for SqlCipherCustomizer {
    fn on_acquire(&self, conn: &mut Connection) -> Result<(), Error> {
        let query = format!("PRAGMA key = '{}';", self.passphrase.replace("'", "''"));
        conn.execute_batch(&query)?;
        conn.execute_batch("PRAGMA foreign_keys = ON")?;
        Ok(())
    }
}

// 2. Build the pool with the customizer
pub fn get_secret_db(
    path: &str,
    passphrase: &str,
) -> Result<Pool<SqliteConnectionManager>, r2d2::Error> {
    let manager = SqliteConnectionManager::file(path);

    let pool = Pool::builder()
        .connection_customizer(Box::new(SqlCipherCustomizer {
            passphrase: passphrase.to_string(),
        }))
        .build(manager)?;

    Ok(pool)
}

pub fn create_secret_db(path: &str, passphrase: &str) -> Result<Connection, DirsqlError> {
    let conn = Connection::open(path)?;
    let query = format!("PRAGMA key = '{}';", passphrase.replace("'", "''"));
    conn.execute_batch(&query)?;

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS secrets (
            id   INTEGER PRIMARY KEY,
            data TEXT NOT NULL
        );
    ",
    )?;

    Ok(conn)
}

pub fn harden(conn: &Connection) -> Result<(), DirsqlError> {
    conn.execute_batch(
        "
        PRAGMA cipher_page_size = 4096;   -- Larger pages = better perf
        PRAGMA kdf_iter = 256000;          -- PBKDF2 iterations (higher = slower brute force)
        PRAGMA cipher_hmac_algorithm = HMAC_SHA512;
        PRAGMA cipher_kdf_algorithm = PBKDF2_HMAC_SHA512;
        PRAGMA cipher_memory_security = ON; -- Wipe memory on free
    ",
    )?;
    Ok(())
}
