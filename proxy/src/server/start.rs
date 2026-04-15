use crate::server::rate_limiter::get_rate_limiter_config;
use crate::server::routes::routes;
use crate::{proxy::create_client::create_client_with_cert, server::log::log_middleware};
use actix_governor::Governor;
use actix_web::{App, HttpServer, web};
use config::CentraleConfig;
use dir_and_db_pool::db::DbBool;
use mio::net::UdpSocket;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

#[actix_web::main]
pub async fn start_server(db: DbBool) -> std::io::Result<()> {
    // RATE LIMITING SETTINGS
    let governor_conf = get_rate_limiter_config();
    // SET UP CONNECTION TO WRITER
    let addr_0: SocketAddr = "0.0.0.0:0".parse().unwrap();
    let socket = UdpSocket::bind(addr_0)?; // OS picks a port
    let socket_arc = Arc::new(Mutex::new(socket));
    let addr: SocketAddr = CentraleConfig::WRITER_SERVER_ADDRESS.parse().unwrap();
    // SSL
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file(CentraleConfig::cert_private_key(), SslFiletype::PEM)?;
    builder.set_certificate_chain_file(CentraleConfig::cert_pub_key())?;
    // CREATE CLIENT WITH CERT
    let client = create_client_with_cert().unwrap();
    // SERVER ITSELF
    HttpServer::new(move || {
        App::new()
            .configure(routes)
            .wrap_fn({
                let socket_2 = socket_arc.clone();
                move |req, srv| log_middleware(req, srv, socket_2.clone(), addr)
            })
            .wrap(Governor::new(&governor_conf))
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(client.clone()))
    })
    .workers(CentraleConfig::PROXY_SERVER_WORKERS)
    .bind_openssl(CentraleConfig::SERVER_ADDRESS, builder)?
    .run()
    .await
}
