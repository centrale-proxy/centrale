use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
};
use config::CentraleConfig;

pub async fn auth_master_bearer_token(
    req: ServiceRequest,
    next: actix_web::middleware::Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    let master_token = CentraleConfig::master_bearer_token();

    match auth_header {
        Some(header) if header == format!("Bearer {}", master_token) => next.call(req).await,
        _ => Err(actix_web::error::ErrorUnauthorized(
            "Master token unauthenticated",
        )),
    }
}
