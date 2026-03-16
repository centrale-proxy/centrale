use crate::{rate_limiter::get_rate_limiter_config, routes::routes};
use actix_governor::Governor;
use actix_web::{App, HttpServer, web};
use config::CentraleConfig;
use dir_and_db_pool::db::DbBool;

#[actix_web::main]
pub async fn start_server(db: DbBool) -> std::io::Result<()> {
    //
    let governor_conf = get_rate_limiter_config();

    HttpServer::new(move || {
        App::new()
            .configure(routes)
            .wrap(Governor::new(&governor_conf))
            .app_data(web::Data::new(db.clone()))
    })
    .bind(CentraleConfig::SERVER_ADDRESS)?
    .run()
    .await
}
