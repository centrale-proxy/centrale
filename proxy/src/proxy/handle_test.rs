use crate::proxy::api_test::api_test;
use actix_web::{HttpResponse, Responder, web};
use dir_and_db_pool::db::DbBool;
use log::error;

pub async fn handle_test(pool: web::Data<DbBool>) -> impl Responder {
    match api_test(pool) {
        Ok(result) => result,
        Err(err) => {
            error!("/api/test error: {}", err);
            HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Test not Ok" }))
        }
    }
}

#[actix_rt::test]
async fn api_test_ok() {
    use crate::proxy::create_test_app::_create_test_app;
    use actix_web::test;

    let app = _create_test_app().await;

    let req = test::TestRequest::get().uri("/api/tester").to_request();

    let resp = test::call_service(&app, req).await;
    //println!("resp {:?}", resp);
    assert!(resp.status().is_success());
}
