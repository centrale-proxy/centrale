use crate::{
    db::{create_subdomain_db, get_subdomain_db},
    error::SampleServerError,
};
use actix_web::{HttpRequest, HttpResponse, web};
use dir_and_db_pool::db::DbBool;
///
pub fn process_register(
    pool: web::Data<DbBool>,
    req: HttpRequest,
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
        let conn = get_subdomain_db(&subdomain_id.unwrap(), &centrale_password.unwrap()).unwrap();
        create_subdomain_db(&conn, centrale_password.unwrap()).unwrap();

        let c = conn.get().unwrap();
        c.execute_batch(&format!("PRAGMA key = '{}';", &centrale_password.unwrap()))
            .unwrap();

        // CREATE SUBDOMAIN DB
        c.execute_batch(&format!(
            "INSERT INTO {} (test) VALUES ('this is test value');",
            subdomain_id.unwrap()
        ))
        .unwrap();

        let resp = HttpResponse::Ok()
            .json(serde_json::json!({ "subdomain_id": subdomain_id.unwrap().to_string(), "password": centrale_password.unwrap().to_string(), "role": customer_role.unwrap().to_string() }));
        Ok(resp)
    } else {
        Err(SampleServerError::StringError("Unauthorized".to_string()))
    }
}
