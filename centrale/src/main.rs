mod config;
mod request;

use crate::{config::CentraleConfig, request::handle_request};
use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| App::new().route("/{_:.*}", web::get().to(handle_request)))
        .bind(CentraleConfig::SERVER_ADDRESS)?
        .run()
        .await
}
