use actix_web::web;
use crate::handlers::auth::{register, login, verify_google_token};

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/google", web::post().to(verify_google_token))
    );
}