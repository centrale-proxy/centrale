use crate::{
    api::subdomain_user::post::logic::logic_subdomain_add_user, server::auth::CentraleUser,
};
use actix_web::{HttpResponse, Responder, web};
use dir_and_db_pool::db::DbPool;
use log::error;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SubdomainUserAndRole {
    pub subdomain: String,
    pub user_id: i64,
    pub role: String,
}

pub async fn subdomain_post_user_role(
    pool: web::Data<DbPool>,
    json: web::Json<SubdomainUserAndRole>,
    user: CentraleUser,
) -> impl Responder {
    //
    match logic_subdomain_add_user(pool, json, user) {
        Ok(result) => result,
        Err(err) => {
            error!("Add subdomain error: {}", err);
            eprintln!(" err {:?}", err);
            HttpResponse::UnprocessableEntity()
                .json(serde_json::json!({ "error": &err.to_string() }))
        }
    }
}
