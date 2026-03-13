//! Integration tests for API route handlers.
//! These tests verify HTTP routing, request parsing, and response structure
//! using Actix Web's test utilities without requiring a live database.

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use actix_web::http::StatusCode;

    use ratemyhackathons_backend::routes;

    /// Helper: creates a test app with all routes registered (no DB pool).
    /// Useful for testing route existence and 404s on wrong routes.
    fn test_app_config(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/api")
                .service(routes::events::list_events)
                .service(routes::events::get_event)
                .service(routes::events::create_event)
                .service(routes::companies::list_companies)
                .service(routes::companies::get_company)
                .service(routes::companies::create_company)
                .service(routes::users::list_users)
                .service(routes::users::get_user)
                .service(routes::users::create_user)
                .service(routes::users::create_review)
                .service(routes::search::search),
        );
    }

    // ── Route existence tests ──
    // These verify the correct HTTP method + path combos are registered.
    // They will return 500 (no DB pool) but NOT 404, proving the route exists.

    #[actix_rt::test]
    async fn events_list_route_exists() {
        let app = test::init_service(App::new().configure(test_app_config)).await;
        let req = test::TestRequest::get().uri("/api/events").to_request();
        let resp = test::call_service(&app, req).await;
        // 500 is expected (no DB pool), but NOT 404
        assert_ne!(resp.status(), StatusCode::NOT_FOUND, "GET /api/events should exist");
    }

    #[actix_rt::test]
    async fn events_get_route_exists() {
        let app = test::init_service(App::new().configure(test_app_config)).await;
        let req = test::TestRequest::get()
            .uri("/api/events/01961e2a-0000-7000-8000-000000000001")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), StatusCode::NOT_FOUND, "GET /api/events/:id should exist");
    }

    #[actix_rt::test]
    async fn events_create_route_exists() {
        let app = test::init_service(App::new().configure(test_app_config)).await;
        let req = test::TestRequest::post()
            .uri("/api/events")
            .set_json(serde_json::json!({"name": "Test"}))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), StatusCode::NOT_FOUND, "POST /api/events should exist");
    }

    #[actix_rt::test]
    async fn companies_list_route_exists() {
        let app = test::init_service(App::new().configure(test_app_config)).await;
        let req = test::TestRequest::get().uri("/api/companies").to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), StatusCode::NOT_FOUND, "GET /api/companies should exist");
    }

    #[actix_rt::test]
    async fn companies_get_route_exists() {
        let app = test::init_service(App::new().configure(test_app_config)).await;
        let req = test::TestRequest::get()
            .uri("/api/companies/01961e2a-0000-7000-8000-000000000001")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), StatusCode::NOT_FOUND, "GET /api/companies/:id should exist");
    }

    #[actix_rt::test]
    async fn companies_create_route_exists() {
        let app = test::init_service(App::new().configure(test_app_config)).await;
        let req = test::TestRequest::post()
            .uri("/api/companies")
            .set_json(serde_json::json!({"name": "Test Co"}))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), StatusCode::NOT_FOUND, "POST /api/companies should exist");
    }

    #[actix_rt::test]
    async fn users_list_route_exists() {
        let app = test::init_service(App::new().configure(test_app_config)).await;
        let req = test::TestRequest::get().uri("/api/users").to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), StatusCode::NOT_FOUND, "GET /api/users should exist");
    }

    #[actix_rt::test]
    async fn users_get_route_exists() {
        let app = test::init_service(App::new().configure(test_app_config)).await;
        let req = test::TestRequest::get()
            .uri("/api/users/01961e2a-0000-7000-8000-000000000001")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), StatusCode::NOT_FOUND, "GET /api/users/:id should exist");
    }

    #[actix_rt::test]
    async fn users_create_route_exists() {
        let app = test::init_service(App::new().configure(test_app_config)).await;
        let req = test::TestRequest::post()
            .uri("/api/users")
            .set_json(serde_json::json!({"username": "test", "email": "t@t.com"}))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), StatusCode::NOT_FOUND, "POST /api/users should exist");
    }

    #[actix_rt::test]
    async fn reviews_create_route_exists() {
        let app = test::init_service(App::new().configure(test_app_config)).await;
        let req = test::TestRequest::post()
            .uri("/api/reviews")
            .set_json(serde_json::json!({
                "event_id": "01961e2a-0000-7000-8000-000000000001",
                "user_id": "01961e2a-0000-7000-8000-000000000002",
                "rating": 5
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), StatusCode::NOT_FOUND, "POST /api/reviews should exist");
    }

    #[actix_rt::test]
    async fn search_route_exists() {
        let app = test::init_service(App::new().configure(test_app_config)).await;
        let req = test::TestRequest::get()
            .uri("/api/search?q=hackathon")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), StatusCode::NOT_FOUND, "GET /api/search should exist");
    }

    // ── Wrong method tests ──

    #[actix_rt::test]
    async fn events_wrong_method_returns_405() {
        let app = test::init_service(App::new().configure(test_app_config)).await;
        let req = test::TestRequest::delete().uri("/api/events").to_request();
        let resp = test::call_service(&app, req).await;
        // Should be 404 or 405 (method not allowed)
        assert!(
            resp.status() == StatusCode::NOT_FOUND || resp.status() == StatusCode::METHOD_NOT_ALLOWED,
            "DELETE /api/events should not be allowed"
        );
    }

    // ── Non-existent route tests ──

    #[actix_rt::test]
    async fn nonexistent_route_returns_404() {
        let app = test::init_service(App::new().configure(test_app_config)).await;
        let req = test::TestRequest::get().uri("/api/nonexistent").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn root_without_api_prefix_returns_404() {
        let app = test::init_service(App::new().configure(test_app_config)).await;
        let req = test::TestRequest::get().uri("/events").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    // ── Search validation tests ──

    #[actix_rt::test]
    async fn search_without_q_param_returns_error() {
        let app = test::init_service(App::new().configure(test_app_config)).await;
        let req = test::TestRequest::get().uri("/api/search").to_request();
        let resp = test::call_service(&app, req).await;
        // Should be 400 (missing required param) or 500 (no DB)
        assert_ne!(resp.status(), StatusCode::OK, "Search without q should not succeed");
    }
}
