use crate::{
    request::handle_wildcard,
    user::{get::get_user, post::post_user},
};
use actix_web::{App, HttpServer, web};
use config::CentraleConfig;
use dir_and_db_pool::db::DbBool;

#[actix_web::main]
pub async fn start_server(db: DbBool) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/api/user", web::post().to(post_user))
            .route("/api/user", web::get().to(get_user))
            .route("/{_:.*}", web::get().to(handle_wildcard))
    })
    .bind(CentraleConfig::SERVER_ADDRESS)?
    .run()
    .await
}
