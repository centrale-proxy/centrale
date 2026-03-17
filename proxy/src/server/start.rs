use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::server::log::log_middleware;
use crate::server::rate_limiter::get_rate_limiter_config;
use crate::server::routes::routes;
use actix_governor::Governor;
use actix_web::{App, HttpServer, web};
use config::CentraleConfig;
use dir_and_db_pool::db::DbBool;

#[actix_web::main]
pub async fn start_server(db: DbBool) -> std::io::Result<()> {
    //
    let governor_conf = get_rate_limiter_config();

    let log_stream = TcpStream::connect("127.0.0.1:8081")?;
    let log_stream = Arc::new(Mutex::new(log_stream));

    HttpServer::new(move || {
        App::new()
            .configure(routes)
            .wrap_fn({
                let stream = log_stream.clone(); // clone the Arc, not the stream
                move |req, srv| log_middleware(req, srv, stream.clone())
            })
            .wrap(Governor::new(&governor_conf))
            .app_data(web::Data::new(db.clone()))
    })
    .bind(CentraleConfig::SERVER_ADDRESS)?
    .run()
    .await
}
