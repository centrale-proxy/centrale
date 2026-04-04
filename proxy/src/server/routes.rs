use crate::{
    proxy::{
        handle_test::handle_test, wildcard::handle_wildcard,
        wildcard_with_payload::handle_wildcard_with_payload,
    },
    subdomain::respond_post::respond_subdomain,
    user::{
        air_token::generate_responder::generate_air_token, get::get::get_user,
        post::post::post_user,
    },
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
        web::resource("/api/user/air/token")
            .route(web::get().to(generate_air_token))
            .route(web::head().to(|| HttpResponse::Ok())),
    );

    cfg.service(
        web::resource("/api/subdomain")
            .route(web::post().to(respond_subdomain))
            .route(web::head().to(|| HttpResponse::Ok())),
    );

    cfg.service(web::resource("/api/tester").route(web::get().to(handle_test)));

    cfg.service(
        web::resource("/{_:.*}")
            .route(web::get().to(handle_wildcard))
            .route(web::delete().to(handle_wildcard))
            .route(web::post().to(handle_wildcard_with_payload))
            .route(web::put().to(handle_wildcard_with_payload))
            .route(web::head().to(|| HttpResponse::Ok())),
    );
}
//
