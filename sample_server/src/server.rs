use crate::pool::DbPoolRegistry;
use crate::{auth::auth_master_bearer_token, error::SampleServerError, routes::routes};
use actix_web::middleware::from_fn;
use actix_web::{App, HttpServer, web};
use config::CentraleConfig;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::sync::RwLock;
use std::{collections::HashMap, sync::Arc};

#[actix_web::main]
pub async fn start_server() -> Result<(), SampleServerError> {
    // CREATE REGISTRY //
    let pools = HashMap::new();
    let registry = Arc::new(RwLock::new(DbPoolRegistry { pools }));
    // SET HTTPS
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(CentraleConfig::cert_private_key(), SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file(CentraleConfig::cert_pub_key())
        .unwrap();

    // ANNOUNCE
    println!(
        "server started at {}",
        CentraleConfig::get("SAMPLE_SERVER_IP")
    );
    //START
    HttpServer::new(move || {
        App::new()
            .configure(routes)
            .wrap(from_fn(auth_master_bearer_token))
            .app_data(web::Data::new(registry.clone()))
    })
    .workers(CentraleConfig::SAMPLE_SERVER_WORKERS)
    .bind_openssl(CentraleConfig::get("SAMPLE_SERVER_IP"), builder)?
    .run()
    .await?;

    Ok(())
}
