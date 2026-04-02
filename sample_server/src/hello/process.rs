use crate::error::SampleServerError;
use actix_web::{HttpRequest, HttpResponse, web};
use dir_and_db_pool::db::DbBool;

//
pub fn process_hello(
    pool: web::Data<DbBool>,
    req: HttpRequest,
) -> Result<HttpResponse, SampleServerError> {
    //
    let customer_id = req
        .headers()
        .get("centrale_id")
        .and_then(|v| v.to_str().ok());

    let customer_token = req
        .headers()
        .get("centrale_token")
        .and_then(|v| v.to_str().ok());

    if customer_token.is_some() && customer_id.is_some() {
        let resp = HttpResponse::Ok()
            .json(serde_json::json!({ "user_id": customer_id.unwrap().to_string(), "token": customer_token.unwrap().to_string() }));
        Ok(resp)
    } else {
        Err(SampleServerError::StringError("Unauthorized".to_string()))
    }
}
