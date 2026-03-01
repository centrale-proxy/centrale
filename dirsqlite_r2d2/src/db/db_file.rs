use crate::{db::app_folder::app_folder, error::DirsqlError};
use std::path::PathBuf;

pub fn db_file(file: &str, folder: &str) -> Result<PathBuf, DirsqlError> {
    let folder_path = app_folder(folder)?;
    Ok(folder_path.join(file))
}

pub fn does_db_exist(file: &str, folder: &str) -> Result<bool, DirsqlError> {
    let file = db_file(file, folder)?;
    Ok(file.exists() && file.is_file())
}
