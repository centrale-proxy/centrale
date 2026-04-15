use crate::error::CentraleError;
use config::CentraleConfig;
use reqwest::{Certificate, Client};
use std::fs;

pub fn create_client_with_cert() -> Result<Client, CentraleError> {
    // ADD CERT
    let cert = fs::read(CentraleConfig::cert_private_key())?;
    let cert = Certificate::from_pem(&cert)?;
    let client = Client::builder().add_root_certificate(cert).build()?;
    Ok(client)
}
