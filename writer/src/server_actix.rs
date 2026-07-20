use crate::error::WriterError;
use crate::routes::routes;
//use crate::standalone::{DbMiddleware, init_pool};
//use actix_files::Files;
use actix_web::{App, HttpServer, web};
use config::CentraleConfig;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
//use std::collections::HashMap;
//use std::sync::Arc;
//use std::sync::RwLock;
//use prompt_client::PromptClient;

pub async fn start_server_actix(
    feed_tx: tokio::sync::broadcast::Sender<String>,
) -> Result<(), WriterError> {
    // SET HTTPS
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(CentraleConfig::cert_private_key(), SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file(CentraleConfig::cert_pub_key())
        .unwrap();
    // CREATE REGISTRY //
    // let pools = HashMap::new();
    // let registry = Arc::new(RwLock::new(DbPoolRegistry { pools }));

    // TEST IF ALL VARS ARE THERE
    // BudgetConfig::test();

    // STANDALONE
    //let pool = init_pool();
    //let db = pool.get().unwrap();
    // INIT DB
    //let _ = init_web_db(&db);
    // ANNOUNCE
    println!("Admin server starting",);
    //START /
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(feed_tx.clone()))
            //  .app_data(web::Data::new(registry.clone()))
            //  .wrap(DbMiddleware::new(pool.clone())) // SINGLE PLAUYER
            //.wrap(from_fn(log_url))
            //  .app_data(web::Data::new(client.clone()))
            //   .app_data(web::Data::new(prompt_client.clone()))
            .configure(routes)
        //.service(
        //      Files::new("/", CentraleConfig::get("ADMIN_WEB_FILES")).index_file("index.html"),
        //  )
    })
    .workers(1)
    // .bind(CentraleConfig::ADMIN_SERVER_ADDRESS)?
    .bind_openssl(CentraleConfig::ADMIN_SERVER_ADDRESS, builder)?
    .run()
    .await?;

    Ok(())
}
