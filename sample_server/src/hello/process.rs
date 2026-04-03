use crate::{
    error::SampleServerError, pool::get::get_or_create_from_registry, server::DbPoolRegistry,
};
use actix_web::{HttpRequest, HttpResponse, web};
use rusqlite::params;
use std::sync::{Arc, RwLock};

pub fn process_hello(
    registry: web::Data<Arc<RwLock<DbPoolRegistry>>>,
    req: HttpRequest,
) -> Result<HttpResponse, SampleServerError> {
    //
    let subdomain_id = req
        .headers()
        .get("centrale_subdomain")
        .and_then(|v| v.to_str().ok());

    let centrale_password = req
        .headers()
        .get("centrale_password")
        .and_then(|v| v.to_str().ok());

    let customer_role = req
        .headers()
        .get("centrale_role")
        .and_then(|v| v.to_str().ok());

    if centrale_password.is_some() && subdomain_id.is_some() && customer_role.is_some() {
        // GET / CREATE + GET CONNECTION
        let conn = get_or_create_from_registry(
            &registry,
            subdomain_id.unwrap(),
            centrale_password.unwrap(),
        )?;

        let db = conn.get().unwrap();
        let mut stmt = db.prepare(&"SELECT data FROM secrets;").unwrap();
        let data: String = stmt.query_row(params![], |row| row.get(0)).unwrap();

        let resp = HttpResponse::Ok()
            .json(serde_json::json!({ "subdomain_id": subdomain_id.unwrap().to_string(), "data": data, "role": customer_role.unwrap().to_string() }));
        Ok(resp)
    } else {
        Err(SampleServerError::StringError("Unauthorized".to_string()))
    }
}
