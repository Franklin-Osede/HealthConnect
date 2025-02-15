// tests/auth_integration_test.rs
use actix_web::{test, App, web};
use serde_json::json;
use sqlx::PgPool;
use HealthConnect::routes; // This now works because src/lib.rs re-exports routes

#[actix_web::test]
async fn integration_test_register() {
    // Set up a connection pool (using a test database URL)
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://test:test@localhost/test_db".to_string());
    let pool = PgPool::connect(&database_url).await.unwrap();

    // Initialize the Actix app with your authentication routes.
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .configure(routes::config)
    ).await;

    let payload = json!({
        "username": "integrationuser",
        "email": "integrationuser@example.com",
        "password": "integrationpassword"
    });

    let req = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "Expected successful registration");
}
