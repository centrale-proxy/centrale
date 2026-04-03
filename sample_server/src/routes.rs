use crate::{hello::respond::respond_hello, register::respond::respond_register};
use actix_web::{HttpResponse, web};
///
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(respond_hello))
            .route(web::head().to(|| HttpResponse::Ok())),
    );

    cfg.service(
        web::resource("/api/register_subdomain").route(web::post().to(respond_register)), // .route(web::head().to(|| HttpResponse::Ok())),
    );
}
