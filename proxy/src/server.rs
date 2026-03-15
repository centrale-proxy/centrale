use crate::{
    request::handle_wildcard,
    subdomain::respond_post::respond_subdomain,
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
            .route("/api/subdomain", web::post().to(respond_subdomain))
            .route("/{_:.*}", web::get().to(handle_wildcard))
    })
    .bind(CentraleConfig::SERVER_ADDRESS)?
    .run()
    .await
}
