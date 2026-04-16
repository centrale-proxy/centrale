#[cfg(test)]
mod tests {
    use crate::proxy::test::create_test_app::_create_test_app;
    use actix_web::test;

    #[actix_rt::test]
    async fn ws_connection_401_unauthorized() {
        // Create test app using the app factory
        dotenvy::dotenv().ok();
        let app = _create_test_app().await;

        // Create a test request without authentication header
        let req = test::TestRequest::get().uri("/air").to_request();

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
    async fn ws_connection_401_with_auth_header() {
        // Create test app using the app factory
        let app = _create_test_app().await;

        // Create a test request with invalid authentication header
        let req = test::TestRequest::get()
            .uri("/ws")
            .insert_header(("Authorization", "Bearer invalid_token"))
            .to_request();

        // Send request to test service
        let resp = test::call_service(&app, req).await;

        // Assert that response is 401 Unauthorized for invalid token
        assert_eq!(
            resp.status(),
            actix_web::http::StatusCode::UNAUTHORIZED,
            "Expected 401 Unauthorized response for invalid token"
        );
    }

    #[actix_rt::test]
    async fn ws_connection_401_multiple_attempts() {
        // Create test app using the app factory
        let app = _create_test_app().await;

        // Try multiple connection attempts without authentication
        for attempt in 1..=3 {
            let req = test::TestRequest::get().uri("/ws").to_request();

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
}
