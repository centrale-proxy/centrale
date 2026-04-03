use crate::hello::respond::respond_hello;
use actix_web::{HttpResponse, web};
///
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(respond_hello))
            .route(web::head().to(|| HttpResponse::Ok())),
    );
}
