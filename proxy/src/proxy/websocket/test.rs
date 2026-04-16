// CREATE USER
// GET COOKIE
// CONNECDT TO WS
// GENERATE TOKEN
// ADD TO BEARER

#[actix_rt::test]
async fn ws_connection_401_unauthorized() {
    use crate::proxy::{
        auth::subdomain_string::_get_centrale_cookie, test::create_test_app::_create_test_app,
        wildcard::test::_user_create_request,
    };
    use actix_web::test;

    dotenvy::dotenv().ok();
    let app = _create_test_app().await;
    let req = _user_create_request();
    let resp = test::call_service(&app, req).await;
    // GET COOKIE
    let cookie_value = _get_centrale_cookie(resp.headers()).unwrap();
    let cookie = format!("centrale={}", cookie_value);

    let req = test::TestRequest::get()
        .uri("/ws")
        .insert_header(("Upgrade", "websocket"))
        .insert_header(("Connection", "Upgrade"))
        .insert_header(("Sec-WebSocket-Key", "dGhlIHNhbXBsZSBub25jZQ=="))
        .insert_header(("Sec-WebSocket-Version", "13"))
        .insert_header(("Cookie", cookie))
        .to_request();

    // Send request to test service
    let resp = test::call_service(&app, req).await;

    // Assert that response is 401 Unauthorized
    assert_eq!(
        resp.status(),
        actix_web::http::StatusCode::OK,
        "Expected 200 OK response"
    );
}

#[actix_rt::test]
async fn test_websocket_connection_401_unauthorized() {
    use crate::proxy::test::create_test_app::_create_test_app;
    use actix_web::test;

    // Create test app using the app factory
    let app = _create_test_app().await;

    // Create a WebSocket upgrade request WITHOUT authentication header
    // These are the standard WebSocket upgrade headers
    let req = test::TestRequest::get()
        .uri("/ws")
        .insert_header(("Upgrade", "websocket"))
        .insert_header(("Connection", "Upgrade"))
        .insert_header(("Sec-WebSocket-Key", "dGhlIHNhbXBsZSBub25jZQ=="))
        .insert_header(("Sec-WebSocket-Version", "13"))
        .to_request();

    // Send request to test service
    let resp = test::call_service(&app, req).await;

    // Assert that response is 401 Unauthorized
    assert_eq!(
        resp.status(),
        actix_web::http::StatusCode::UNAUTHORIZED,
        "Expected 401 Unauthorized response"
    );
}

#[actix_rt::test]
async fn test_websocket_connection_401_with_auth_header() {
    use crate::proxy::test::create_test_app::_create_test_app;
    use actix_web::test;

    // Create test app using the app factory
    let app = _create_test_app().await;

    // Create a WebSocket upgrade request with invalid authentication header
    // WebSocket upgrade requires specific headers
    let req = test::TestRequest::get()
        .uri("/ws")
        .insert_header(("Upgrade", "websocket"))
        .insert_header(("Connection", "Upgrade"))
        .insert_header(("Sec-WebSocket-Key", "dGhlIHNhbXBsZSBub25jZQ=="))
        .insert_header(("Sec-WebSocket-Version", "13"))
        .insert_header(("Authorization", "Bearer invalid_token"))
        .to_request();

    // Send request to test service
    let resp = test::call_service(&app, req).await;

    // Assert that response is 401 Unauthorized for invalid token
    // (should be checked BEFORE the WebSocket upgrade happens)
    assert_eq!(
        resp.status(),
        actix_web::http::StatusCode::UNAUTHORIZED,
        "Expected 401 Unauthorized response for invalid token"
    );
}

#[actix_rt::test]
async fn test_websocket_connection_401_multiple_attempts() {
    use crate::proxy::test::create_test_app::_create_test_app;
    use actix_web::test;

    // Create test app using the app factory
    let app = _create_test_app().await;

    // Try multiple WebSocket connection attempts without authentication
    for attempt in 1..=3 {
        let req = test::TestRequest::get()
            .uri("/ws")
            .insert_header(("Upgrade", "websocket"))
            .insert_header(("Connection", "Upgrade"))
            .insert_header(("Sec-WebSocket-Key", "dGhlIHNhbXBsZSBub25jZQ=="))
            .insert_header(("Sec-WebSocket-Version", "13"))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(
            resp.status(),
            actix_web::http::StatusCode::UNAUTHORIZED,
            "Attempt {}: Expected 401 Unauthorized response",
            attempt
        );
        println!("Attempt {}: Got expected 401 Unauthorized", attempt);
    }
}
