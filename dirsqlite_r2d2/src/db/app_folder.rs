use crate::{db::create_home_dir::create_home_dir, error::DirsqlError};
use dirs::data_dir;
use std::path::PathBuf;

pub fn app_folder(folder: &str) -> Result<PathBuf, DirsqlError> {
    let path = data_dir()
        .ok_or_else(|| DirsqlError::StringError("Unable to find main directory".into()))?;
    let company_db_path = path.join(folder);

    if !company_db_path.exists() {
        create_home_dir(company_db_path.clone())?;
    }

    Ok(company_db_path)
}
