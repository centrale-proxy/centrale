use crate::proxy::auth::authenticate_and_authorize::authenticate_and_authorize;
use crate::server::auth::CentraleUser;
use actix_web::body::{BoxBody, EitherBody};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error, HttpMessage, error::ErrorUnauthorized, web};
use dir_and_db_pool::db::DbBool;

pub async fn auth_middleware_2(
    mut req: ServiceRequest,
    srv: actix_web::middleware::Next<BoxBody>,
) -> Result<ServiceResponse<EitherBody<BoxBody>>, Error> {
    let pool = req
        .app_data::<web::Data<DbBool>>()
        .cloned()
        .ok_or_else(|| ErrorUnauthorized("Database pool not available"))?;
    match authenticate_and_authorize(pool, req.request()) {
        Ok((user_id, subdomain, role, pass, url)) => {
            let user = CentraleUser {
                user_id,
                subdomain,
                role,
                pass,
                url,
            };
            req.extensions_mut().insert(user);
            srv.call(req).await.map(|res| res.map_into_right_body())
        }
        Err(e) => Err(ErrorUnauthorized("Unauthorized")),
    }
}
