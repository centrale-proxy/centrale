use crate::{config::CentraleConfig, request::handle_request};
use actix_web::{App, HttpServer, web};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

#[actix_web::main]
pub async fn start_server(db: Pool<SqliteConnectionManager>) -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/{_:.*}", web::get().to(handle_request)))
        .bind(CentraleConfig::SERVER_ADDRESS)?
        .run()
        .await
}
