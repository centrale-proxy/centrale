use actix_web::{HttpResponse, web};
///
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(respond_test))
            .route(web::head().to(|| HttpResponse::Ok())),
    );
}

pub async fn respond_test() -> HttpResponse {
    let body = "

        hello
        ";
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(body)
}
