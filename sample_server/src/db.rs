use crate::{error::SampleServerError, register::subdomain_path::create_subdomain_path};
use config::CentraleConfig;
use dir_and_db_pool::{
    db::{
        DbBool,
        db_file::db_file,
        encrypted::{create_secret_db, get_secret_db},
    },
    error::DirsqlError,
};

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
    let path = create_subdomain_path(subdomain)?;
    let conn = get_secret_db(&path, pass).unwrap();

    Ok(conn)
}

pub fn create_subdomain_db(conn: &DbBool, pass: &str) -> Result<(), DirsqlError> {
    // let conn = Connection::open(path)?;
    // conn.execute_batch(&format!("PRAGMA key = '{}';", passphrase))?;
    let db = conn.get().unwrap();
    db.execute_batch(&format!("PRAGMA key = '{}';", pass))?;
    db.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS secrets (
            id   INTEGER PRIMARY KEY,
            data TEXT NOT NULL
        );
    ",
    )?;

    Ok(())
}
