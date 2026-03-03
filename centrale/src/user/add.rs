use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub password: String,
}

pub async fn add_user(json: web::Json<RegisterUser>) -> impl Responder {
    let register_request = json.into_inner();
    let username = register_request.username;
    let _password = register_request.password;

    HttpResponse::Ok().body(format!("User {} registered successfully!", username))
}

#[actix_rt::test]
async fn post_new_user() {
    use actix_web::{App, test, web};
    let app = test::init_service(App::new().route("/api/user", web::post().to(add_user))).await;
    use serde_json::json;

    let payload = json!({
        "username": "testuser",
        "password": "testpassword"
    });

    let req = test::TestRequest::get()
        .uri("/api/user")
        .insert_header(("Host", "https://hello.hello.ee"))
        .insert_header(("Content-Type", "application/json"))
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
