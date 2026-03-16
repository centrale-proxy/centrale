use crate::{
    proxy::handle_test::handle_test,
    request::handle_wildcard,
    subdomain::respond_post::respond_subdomain,
    user::{get::get_user, post::post::post_user},
};
use actix_web::{HttpResponse, web};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api/user")
            .route(web::post().to(post_user))
            .route(web::get().to(get_user))
            .route(web::head().to(|| HttpResponse::Ok())),
    );

    cfg.service(
        web::resource("/api/subdomain")
            .route(web::post().to(respond_subdomain))
            .route(web::head().to(|| HttpResponse::Ok())),
    );

    cfg.service(web::resource("/api/test").route(web::get().to(handle_test)));

    cfg.service(
        web::resource("/{_:.*}")
            .route(web::get().to(handle_wildcard))
            .route(web::head().to(|| HttpResponse::Ok())),
    );
}
//
