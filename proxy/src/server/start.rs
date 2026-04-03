use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use mio::net::UdpSocket;

use crate::server::log::log_middleware;
use crate::server::rate_limiter::get_rate_limiter_config;
use crate::server::routes::routes;
use actix_governor::Governor;
use actix_web::{App, HttpServer, web};
use config::CentraleConfig;
use dir_and_db_pool::db::DbBool;
//
#[actix_web::main]
pub async fn start_server(db: DbBool) -> std::io::Result<()> {
    let governor_conf = get_rate_limiter_config();

    let addr_0: SocketAddr = "0.0.0.0:0".parse().unwrap();
    let socket = UdpSocket::bind(addr_0)?; // OS picks a port

    let socket_1 = Arc::new(Mutex::new(socket));

    let addr: SocketAddr = CentraleConfig::WRITER_SERVER_ADDRESS.parse().unwrap();

    HttpServer::new(move || {
        App::new()
            .configure(routes)
            .wrap_fn({
                let socket_2 = socket_1.clone();
                move |req, srv| log_middleware(req, srv, socket_2.clone(), addr)
            })
            .wrap(Governor::new(&governor_conf))
            .app_data(web::Data::new(db.clone()))
    })
    .workers(CentraleConfig::PROXY_SERVER_WORKERS)
    .bind(CentraleConfig::SERVER_ADDRESS)?
    .run()
    .await
}
