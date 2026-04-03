use crate::pool::DbPoolRegistry;
use crate::{auth::auth_master_bearer_token, error::SampleServerError, routes::routes};
use actix_web::middleware::from_fn;
use actix_web::{App, HttpServer, web};
use config::CentraleConfig;
use std::sync::RwLock;
use std::{collections::HashMap, sync::Arc};

#[actix_web::main]
pub async fn start_server() -> Result<(), SampleServerError> {
    // CREATE REGISTRY
    let pools = HashMap::new();
    let registry = Arc::new(RwLock::new(DbPoolRegistry { pools }));
    // ANNOUNCE
    println!(
        "server started at {}",
        CentraleConfig::SAMPLE_SERVER_ADDRESS
    );
    //START
    HttpServer::new(move || {
        App::new()
            .configure(routes)
            .wrap(from_fn(auth_master_bearer_token))
            .app_data(web::Data::new(registry.clone()))
    })
    .bind(CentraleConfig::SAMPLE_SERVER_ADDRESS)?
    .run()
    .await?;

    Ok(())
}
