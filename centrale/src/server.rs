use crate::{config::CentraleConfig, request::handle_wildcard};
use actix_web::{App, HttpServer, web};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

#[actix_web::main]
pub async fn start_server(db: Pool<SqliteConnectionManager>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/{_:.*}", web::get().to(handle_wildcard))
    })
    .bind(CentraleConfig::SERVER_ADDRESS)?
    .run()
    .await
}
