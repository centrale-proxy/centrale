use rusqlite::{Connection, Result};

pub fn get_secret_db(path: &str, passphrase: &str) -> Result<Connection> {
    let conn = Connection::open(path)?;

    // MUST be the very first pragma before any other operations
    conn.execute_batch(&format!("PRAGMA key = '{}';", passphrase))?;

    // Verify it works (will fail if wrong key)
    conn.execute_batch("SELECT count(*) FROM sqlite_master;")?;

    Ok(conn)
}

pub fn create_secret_db(path: &str, passphrase: &str) -> Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch(&format!("PRAGMA key = '{}';", passphrase))?;

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

pub fn harden(conn: &Connection) -> Result<()> {
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
