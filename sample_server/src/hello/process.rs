use crate::{db::get_subdomain_db, error::SampleServerError};
use actix_web::{HttpRequest, HttpResponse, web};
use dir_and_db_pool::db::DbBool;
use rusqlite::params;

pub fn process_hello(
    pool: web::Data<DbBool>,
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
        let conn = get_subdomain_db(&subdomain_id.unwrap(), &centrale_password.unwrap()).unwrap();

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
