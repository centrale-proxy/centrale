use crate::{
    server::auth::CentraleUser,
    subdomain::handle_post::{RegisterSubdomain, handle_post},
};
use actix_web::{HttpResponse, Responder, web};
use dir_and_db_pool::db::DbPool;
use log::error;

pub async fn respond_subdomain(
    pool: web::Data<DbPool>,
    json: web::Json<RegisterSubdomain>,
    client: web::Data<reqwest::Client>,
    user: CentraleUser,
) -> impl Responder {
    //
    match handle_post(pool, json, client, user).await {
        Ok(result) => result,
        Err(err) => {
            error!("Add subdomain error: {}", err);
            eprintln!(" err {:?}", err);

            HttpResponse::UnprocessableEntity()
                .json(serde_json::json!({ "error": "Cannot post subdomain" }))
        }
    }
}
