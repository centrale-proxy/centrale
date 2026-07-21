use crate::db::get_bytes;
use crate::server_actix::BytesDbPool;
use actix_web::{HttpResponse, web};

pub async fn bytes_by_x_id(
    x_id: web::Path<String>,
    bytes_pool: web::Data<BytesDbPool>,
) -> HttpResponse {
    let db = match bytes_pool.0.get() {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Failed to get bytes database connection: {error}");
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "database_unavailable" }));
        }
    };
    match get_bytes(&db, &x_id) {
        Ok(Some(bytes)) => {
            if !bytes.is_ascii() {
                eprintln!("Bytes for x_id '{}' contain non-ASCII data", x_id.as_str());
                return HttpResponse::InternalServerError()
                    .json(serde_json::json!({ "error": "invalid_ascii_data" }));
            }
            // Safe: all bytes are ASCII, therefore valid UTF-8
            let ascii = String::from_utf8(bytes).expect("ASCII bytes are always valid UTF-8");
            HttpResponse::Ok().json(serde_json::json!({ "ascii": ascii }))
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "bytes_not_found",
            "x_id": x_id.into_inner(),
        })),
        Err(error) => {
            eprintln!("Failed to read bytes for x_id '{}': {error}", x_id.as_str());
            HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "database_error" }))
        }
    }
}
