use crate::proxy::auth::authenticate_and_authorize::authenticate_and_authorize;
use crate::server::auth::CentraleUser;
use actix_web::HttpResponse;
use actix_web::body::{EitherBody, MessageBody};
use actix_web::dev::{ConnectionInfo, ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::{Error, HttpMessage, error::ErrorUnauthorized, web};
use dir_and_db_pool::db::DbPool;

pub async fn auth_middleware_2<B>(
    conn: ConnectionInfo, // extractors FIRST
    req: ServiceRequest,  // then req
    next: Next<B>,        // Next LAST
) -> Result<ServiceResponse<EitherBody<B>>, Error>
where
    B: MessageBody + 'static,
{
    let pool = req
        .app_data::<web::Data<DbPool>>()
        .cloned()
        .ok_or_else(|| ErrorUnauthorized("Database pool not available"))?;

    match authenticate_and_authorize(pool, req.request(), conn.host()) {
        Ok((user_id, subdomain, role, pass, url, destination_bearer)) => {
            req.extensions_mut().insert(CentraleUser {
                user_id,
                subdomain,
                role,
                pass,
                url,
                destination_bearer,
            });
            let res = next.call(req).await?;
            Ok(res.map_into_left_body()) // service body -> LEFT
        }
        Err(e) => {
            log::error!("auth err {}", e);
            let (http_req, _) = req.into_parts();
            let response = HttpResponse::Unauthorized()
                .json(serde_json::json!({ "error": "Unauthorized" }))
                .map_into_right_body(); // our body -> RIGHT
            Ok(ServiceResponse::new(http_req, response))
        }
    }
}
