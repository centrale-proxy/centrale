use crate::config::CentraleConfig;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};

async fn handle_db_error() -> impl Responder {
    HttpResponse::Ok().body("unable to db")
}

#[actix_web::main]
pub async fn serve_db_error() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handle_db_error))
            .route("/{_:.*}", web::get().to(handle_db_error))
    })
    .bind(CentraleConfig::SERVER_ADDRESS)?
    .run()
    .await
}
