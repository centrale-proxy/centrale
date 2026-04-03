use crate::error::SampleServerError;
use config::CentraleConfig;
use dir_and_db_pool::db::db_file::db_file;

///
pub fn create_subdomain_path(subdomain: &str) -> Result<String, SampleServerError> {
    let folder = format!("{}/subdomains", CentraleConfig::DB_FOLDER);
    // TBD MAKE SURE FILE PATH DOES NOT EXIST ALREADY
    let file_path = db_file(subdomain, &folder).unwrap();
    let path = file_path.to_str().unwrap();
    Ok(path.to_string())
}
