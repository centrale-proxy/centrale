use crate::error::SampleServerError;
use config::CentraleConfig;
use dir_and_db_pool::db::{
    DbBool,
    db_file::db_file,
    encrypted::{create_secret_db, get_secret_db},
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
