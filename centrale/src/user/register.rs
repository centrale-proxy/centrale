use crate::error::CentraleError;
use actix_web::web;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub password: String,
}

pub fn handle_register(json: web::Json<RegisterUser>) -> Result<String, CentraleError> {
    // Extract the data from the JSON payload
    let register_request = json.into_inner();
    let username = register_request.username;
    let _password = register_request.password;
    Ok(format!("User {} registered successfully!", username))
}

#[actix_rt::test]
async fn post_new_user() {
    use crate::user::post::post_user;
    use actix_web::{App, test, web};

    let app = test::init_service(App::new().route("/api/user", web::post().to(post_user))).await;
    use serde_json::json;

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
    assert!(resp.status().is_success());
}
