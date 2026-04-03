use crate::{db::get_db::get_encrypted_connection, error::CentraleError};
use actix_web::{HttpResponse, web};
use config::CentraleConfig;
use dir_and_db_pool::db::DbBool;
use r2d2_sqlite::rusqlite::params;

/// Query, if server is up
pub fn api_test(pool: web::Data<DbBool>) -> Result<HttpResponse, CentraleError> {
    let db = get_encrypted_connection(pool.get_ref())?;
    let mut stmt = db.prepare(&"SELECT COUNT(*) FROM subdomain LIMIT 1")?;
    let _subdomain: i64 = stmt.query_row(params![], |row| row.get(0))?;
    let res = HttpResponse::Ok().json(serde_json::json!({ "Ok": true }));
    Ok(res)
}
