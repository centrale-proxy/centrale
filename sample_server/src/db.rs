use crate::error::SampleServerError;
use config::CentraleConfig;
use dir_and_db_pool::{
    db::{
        DbBool,
        db_file::db_file,
        encrypted::{create_secret_db, get_secret_db},
    },
    error::DirsqlError,
};
use rusqlite::Connection;

pub fn get_sample_db() -> Result<DbBool, SampleServerError> {
    let file_path = db_file("test_server", CentraleConfig::DB_FOLDER).unwrap();
    let path = file_path.to_str().unwrap();
    // CREATE DB
    create_secret_db(&path, "pass").unwrap();
    // GET CONNECDTION
    let conn = get_secret_db(&path, "pass").unwrap();

    Ok(conn)
}

pub fn get_subdomain_db(subdomain: &str, pass: &str) -> Result<DbBool, SampleServerError> {
    let folder = format!("{}/subdomains", CentraleConfig::DB_FOLDER);
    let file_path = db_file(subdomain, &folder).unwrap();
    let path = file_path.to_str().unwrap();
    // CREATE DB
    create_subdomain_db(&path, pass).unwrap();
    // GET CONNECDTION
    let conn = get_secret_db(&path, pass).unwrap();

    Ok(conn)
}

pub fn create_subdomain_db(path: &str, passphrase: &str) -> Result<Connection, DirsqlError> {
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
