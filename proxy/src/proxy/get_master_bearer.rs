use crate::error::CentraleError;
use config::CentraleConfig;
use log::error;
use std::env;

pub fn get_master_bearer_token() -> Result<String, CentraleError> {
    let master_token = match env::var(CentraleConfig::CENTRALE_MASTER_BEARER_TOKEN) {
        Ok(token) => token,
        Err(err) => {
            error!("{}", err);
            return Err(CentraleError::MissingMasterBearerToken);
        }
    };
    Ok(master_token)
}
