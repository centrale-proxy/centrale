#[actix_rt::test]
async fn ws_connection_200() {
    use crate::proxy::{
        auth::subdomain_string::_get_centrale_cookie, test::create_test_app::_create_test_app,
        wildcard::test::_user_create_request,
    };
    use crate::subdomain::handle_post::_make_register_subdomain_request;
    use actix_web::test;
    use config::CentraleConfig;
    use serde_json::json;

    dotenvy::dotenv().ok();

    let app = _create_test_app().await;
    let host = CentraleConfig::get("DOMAIN");
    let host_s = format!("https://app.{}", host);
    let req = _user_create_request(&host_s);
    let resp = test::call_service(&app, req).await;
    // GET COOKIE
    println!("coo {:?}", &resp);
    let cookie_value = _get_centrale_cookie(resp.headers()).unwrap();
    let cookie = format!("centrale={}", cookie_value);

    let register_subdomain_payload = json!({
        "subdomain": "test",
    });

    let sub_reg =
        _make_register_subdomain_request(register_subdomain_payload, &app, &cookie, &host_s).await;

    assert!(sub_reg.status().is_success());

    let host_s_2 = format!("wss://test.{}", host);

    let req = test::TestRequest::get()
        .uri("/air")
        .insert_header(("Host", host_s_2))
        .insert_header(("Upgrade", "websocket"))
        .insert_header(("Connection", "Upgrade"))
        .insert_header(("Sec-WebSocket-Key", "dGhlIHNhbXBsZSBub25jZQ=="))
        .insert_header(("Sec-WebSocket-Version", "13"))
        .insert_header(("Cookie", cookie_value))
        .to_request();

    // Send request to test service
    let resp = test::try_call_service(&app, req).await;

    match resp {
        Ok(re) => {
            println!("resp {:?}", &re);

            // Assert that response is 401 Unauthorized
            assert_eq!(
                re.status(),
                actix_web::http::StatusCode::from_u16(101).unwrap(),
                "Expected 101 OK response"
            );
        }
        Err(err) => {
            assert_eq!(err.to_string(), "Unauthorized");
        }
    }
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
