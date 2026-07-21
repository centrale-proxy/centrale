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
        Ok(Some(bytes)) => HttpResponse::Ok().json(serde_json::json!({ "bytes": bytes })),
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
