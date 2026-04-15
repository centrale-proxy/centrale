use crate::{
    proxy::{
        handle_test::handle_test, wildcard::handle_wildcard,
        wildcard_with_payload::handle_wildcard_with_payload,
    },
    server::public_rate_limiter::public_rate_limiter_config,
    subdomain::respond_post::respond_subdomain,
    user::{
        air_token::generate_responder::generate_air_token,
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
        web::resource("/api/user")
            .wrap(Governor::new(&public_governor_conf))
            .route(web::get().to(get_user))
            .route(web::head().to(|| HttpResponse::Ok()))
            .route(web::post().to(post_user)),
    );

    cfg.service(
        web::resource("/api/user/{user_id}/bearer/generate")
            .route(web::get().to(generate_bearer_token))
            .route(web::head().to(|| HttpResponse::Ok())),
    );

    cfg.service(
        web::resource("/api/user/{user_id}/bearer/view")
            .route(web::get().to(view_bearer_tokens))
            .route(web::head().to(|| HttpResponse::Ok())),
    );

    cfg.service(
        web::resource("/api/user/air/token")
            .route(web::get().to(generate_air_token))
            .route(web::head().to(|| HttpResponse::Ok())),
    );

    cfg.service(
        web::resource("/api/login")
            .wrap(Governor::new(&public_governor_conf))
            .route(web::post().to(handle_login))
            .route(web::head().to(|| HttpResponse::Ok())),
    );

    cfg.service(
        web::resource("/api/subdomain")
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
            .route(web::get().to(handle_wildcard))
            .route(web::delete().to(handle_wildcard))
            .route(web::post().to(handle_wildcard_with_payload))
            .route(web::put().to(handle_wildcard_with_payload))
            .route(web::head().to(|| HttpResponse::Ok())),
    );
}
//
