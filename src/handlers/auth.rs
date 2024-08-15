use actix_web::{web, HttpResponse, Responder};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::models::auth::{Claims, LoginRequest};

pub async fn login(login: web::Json<LoginRequest>) -> impl Responder {
    // In a real application, validate credentials against a database
    if login.username == "admin" && login.password == "password" {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(2))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: login.username.clone(),
            exp: expiration as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("secret".as_ref()),
        )
        .unwrap();

        HttpResponse::Ok().json(serde_json::json!({ "token": token }))
    } else {
        HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Invalid credentials" }))
    }
}