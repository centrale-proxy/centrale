use std::fs;
use std::path::PathBuf;

use crate::error::DirsqlError;

pub fn create_home_dir(company_db_path: PathBuf) -> Result<PathBuf, DirsqlError> {
    fs::create_dir_all(&company_db_path)?;
    Ok(company_db_path)
}
