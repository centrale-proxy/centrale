use crate::error::SampleServerError;
use actix_web::{HttpRequest, HttpResponse, web};
use dir_and_db_pool::db::DbBool;

pub fn process_hello(
    pool: web::Data<DbBool>,
    req: HttpRequest,
) -> Result<HttpResponse, SampleServerError> {
    //
    let subdomain_id = req
        .headers()
        .get("centrale_subdomain")
        .and_then(|v| v.to_str().ok());

    let customer_token = req
        .headers()
        .get("centrale_password")
        .and_then(|v| v.to_str().ok());

    let customer_role = req
        .headers()
        .get("centrale_role")
        .and_then(|v| v.to_str().ok());

    if customer_token.is_some() && subdomain_id.is_some() {
        let resp = HttpResponse::Ok()
            .json(serde_json::json!({ "subdomain_id": subdomain_id.unwrap().to_string(), "password": customer_token.unwrap().to_string(), "role": customer_role.unwrap().to_string() }));
        Ok(resp)
    } else {
        Err(SampleServerError::StringError("Unauthorized".to_string()))
    }
}
