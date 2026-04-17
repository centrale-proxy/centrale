use crate::{
    proxy::{
        test::handle_test::handle_test, wildcard::handle_wildcard,
        wildcard_with_payload::handle_wildcard_with_payload,
    },
    server::{auth_2::auth_middleware_2, public_rate_limiter::public_rate_limiter_config},
    subdomain::respond_post::respond_subdomain,
    user::{
        bearer_token::responder::generate_bearer_token,
        bearer_token_view::responder::view_bearer_tokens, get::get::get_user,
        login::handle::handle_login, post::post::post_user,
    },
};
use actix_governor::Governor;
use actix_web::{HttpResponse, web};

pub fn routes(cfg: &mut web::ServiceConfig) {
    let public_governor_conf = public_rate_limiter_config();

    cfg.service(
        web::resource("/api/user_add")
            .wrap(Governor::new(&public_governor_conf))
            .route(web::post().to(post_user))
            .route(web::head().to(|| HttpResponse::Ok())),
    );

    cfg.service(
        web::resource("/api/user")
            .wrap(actix_web::middleware::from_fn(auth_middleware_2))
            .wrap(Governor::new(&public_governor_conf))
            .route(web::get().to(get_user))
            .route(web::head().to(|| HttpResponse::Ok())),
    );

    cfg.service(
        web::resource("/api/user/{user_id}/bearer/generate")
            .wrap(actix_web::middleware::from_fn(auth_middleware_2))
            .route(web::get().to(generate_bearer_token))
            .route(web::head().to(|| HttpResponse::Ok())),
    );

    cfg.service(
        web::resource("/api/user/{user_id}/bearer/view")
            .wrap(actix_web::middleware::from_fn(auth_middleware_2))
            .route(web::get().to(view_bearer_tokens))
            .route(web::head().to(|| HttpResponse::Ok())),
    );

    cfg.service(
        // NO AUTH FOR POST
        web::resource("/api/login")
            .wrap(Governor::new(&public_governor_conf))
            .route(web::post().to(handle_login))
            .route(web::head().to(|| HttpResponse::Ok())),
    );

    cfg.service(
        web::resource("/api/subdomain")
            .wrap(actix_web::middleware::from_fn(auth_middleware_2))
            .wrap(Governor::new(&public_governor_conf))
            .route(web::post().to(respond_subdomain))
            .route(web::head().to(|| HttpResponse::Ok())),
    );

    cfg.service(
        web::resource("/api/tester")
            .wrap(Governor::new(&public_governor_conf))
            .route(web::get().to(handle_test)),
    );

    cfg.service(
        web::resource("/{_:.*}")
            .wrap(actix_web::middleware::from_fn(auth_middleware_2))
            .route(web::get().to(handle_wildcard))
            .route(web::delete().to(handle_wildcard))
            .route(web::post().to(handle_wildcard_with_payload))
            .route(web::put().to(handle_wildcard_with_payload))
            .route(web::head().to(|| HttpResponse::Ok())),
    );
}
//
