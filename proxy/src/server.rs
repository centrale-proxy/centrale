use crate::routes::routes;
use actix_web::{App, HttpServer, web};
use config::CentraleConfig;
use dir_and_db_pool::db::DbBool;

#[actix_web::main]
pub async fn start_server(db: DbBool) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .configure(routes)
            .app_data(web::Data::new(db.clone()))
    })
    .bind(CentraleConfig::SERVER_ADDRESS)?
    .run()
    .await
}
