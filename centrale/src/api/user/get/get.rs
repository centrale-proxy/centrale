use crate::{
    api::user::get::get_user::get_user_from_db, db::get_db::get_centrale_db,
    server::auth::CentraleUser,
};
use actix_web::{HttpResponse, Responder, web};
use dir_and_db_pool::db::DbPool;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetUser {
    pub id: i64,
    pub username: String,
    pub name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub personal_code: Option<String>,
    pub email: Option<String>,
}

pub async fn get_user(pool: web::Data<DbPool>, centrale_user: CentraleUser) -> impl Responder {
    match get_centrale_db(pool.get_ref()) {
        Ok(db) => {
            match get_user_from_db(&db, centrale_user.user_id) {
                Ok(user) => {
                    // GET USER NAME
                    HttpResponse::Ok().json(serde_json::json!(user))
                }
                Err(_err) => HttpResponse::InternalServerError()
                    .json(serde_json::json!({"error": "Unable to find user"})),
            }
        }
        Err(_err) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": "Unable to DB"}))
        }
    }
}
