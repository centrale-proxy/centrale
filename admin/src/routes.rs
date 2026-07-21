use actix_web::{HttpResponse, web};

use crate::api::{bytes::bytes_by_x_id, feed::feed};
/// Main router
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(respond_test))
            .route(web::head().to(|| HttpResponse::Ok())),
    )
    .service(web::resource("/api/feed").route(web::get().to(feed)))
    .service(web::resource("/api/bytes/{x_id}").route(web::get().to(bytes_by_x_id)));
}

pub async fn respond_test() -> HttpResponse {
    let body = "

        hello
        ";
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(body)
}
