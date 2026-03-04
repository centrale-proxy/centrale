use crate::{db::init::init_db, error::CentraleError, user::add::add_user};
use actix_web::web;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub password: String,
}

pub fn handle_register(
    pool: web::Data<Pool<SqliteConnectionManager>>,
    json: web::Json<RegisterUser>,
) -> Result<String, CentraleError> {
    let register_request = json.into_inner();
    let username = register_request.username;
    let password = register_request.password;
    init_db(&pool).unwrap();
    let db = pool.get().expect("Couldn't get db connection from pool");
    let user_id = add_user(&db, &username, &password);
    // return user id
    Ok(format!(
        "User {} ({:?}) registered successfully!",
        username, user_id
    ))
}

#[actix_rt::test]
async fn post_new_user() {
    use crate::user::post::post_user;
    use actix_web::{App, test, web};
    use serde_json::json;

    let manager = SqliteConnectionManager::memory();
    let pool = Pool::new(manager).expect("Failed to create pool.");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .route("/api/user", web::post().to(post_user)),
    )
    .await;

    let payload = json!({
        "username": "testuser",
        "password": "testpassword"
    });

    let req = test::TestRequest::post()
        .uri("/api/user")
        .insert_header(("Content-Type", "application/json"))
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    // println!("{:?}", resp);
    assert!(resp.status().is_success());
}
