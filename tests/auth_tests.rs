use actix_web::{test, web, App};
use sqlx::PgPool;
use soctive-zeus-core::{
    routes,
    models::auth::{LoginRequest, LoginResponse},
    handlers::auth::{register, login, verify_google_token}
};

mod common;

#[actix_rt::test]
async fn test_register_integration() {
    let pool = common::create_test_database().await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/register", web::post().to(register))
    ).await;

    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(&LoginRequest {
            username: "testuser".to_string(),
            password: "testpass".to_string(),
        })
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());

    let body: LoginResponse = test::read_body_json(resp).await;
    assert!(!body.token.is_empty());
    assert!(!body.user_id.is_empty());
}

#[actix_rt::test]
async fn test_login_integration() {
    let pool = common::create_test_database().await;
    
    // First, register a user
    let _: User = sqlx::query_as(
        "INSERT INTO users (id, username, email, password_hash) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind("testuser")
    .bind("testuser@example.com")
    .bind(&bcrypt::hash("testpass", 7).unwrap())
    .fetch_one(&pool)
    .await
    .expect("Failed to create test user");

    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/login", web::post().to(login))
    ).await;

    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&LoginRequest {
            username: "testuser".to_string(),
            password: "testpass".to_string(),
        })
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());

    let body: LoginResponse = test::read_body_json(resp).await;
    assert!(!body.token.is_empty());
    assert!(!body.user_id.is_empty());
}

#[actix_rt::test]
async fn test_auth_flow() {
    let pool = common::create_test_database().await;

    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::auth::auth_routes)
    ).await;

    // Test registration
    let register_req = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(&LoginRequest {
            username: "integrationuser".to_string(),
            password: "integrationpass".to_string(),
        })
        .to_request();

    let register_resp = test::call_service(&mut app, register_req).await;
    assert!(register_resp.status().is_success());

    // Test login
    let login_req = test::TestRequest::post()
        .uri("/auth/login")
        .set_json(&LoginRequest {
            username: "integrationuser".to_string(),
            password: "integrationpass".to_string(),
        })
        .to_request();

    let login_resp = test::call_service(&mut app, login_req).await;
    assert!(login_resp.status().is_success());

    // You can add more integration tests here, such as testing protected routes, etc.
}