use crate::{
    db::create_subdomain_db,
    error::SampleServerError,
    pool::{DbPoolRegistry, get::get_or_create_from_registry},
};
use actix_web::{HttpRequest, HttpResponse, web};
use std::sync::{Arc, RwLock};
///
pub fn process_register(
    registry: web::Data<Arc<RwLock<DbPoolRegistry>>>,
    req: HttpRequest,
    // BODY
) -> Result<HttpResponse, SampleServerError> {
    //
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
        // TRY TO REGISTER DB
        // GET CONNECDTION
        let conn = get_or_create_from_registry(
            &registry,
            subdomain_id.unwrap(),
            centrale_password.unwrap(),
        )?;
        //
        create_subdomain_db(&conn, centrale_password.unwrap()).unwrap();

        let db = conn.get().unwrap();

        let query = format!(
            "PRAGMA key = '{}';",
            centrale_password.unwrap().replace("'", "''")
        );

        db.execute_batch(&query).unwrap();

        // CREATE SUBDOMAIN DB
        db.execute_batch(&format!(
            "INSERT INTO secrets (data) VALUES ('this is test value');",
        ))
        .unwrap();

        let resp = HttpResponse::Ok()
            .json(serde_json::json!({ "subdomain_id": subdomain_id.unwrap().to_string(), "password": centrale_password.unwrap().to_string(), "role": customer_role.unwrap().to_string() }));
        Ok(resp)
    } else {
        Err(SampleServerError::StringError("Unauthorized".to_string()))
    }
}
