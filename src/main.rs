use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod config;
mod routes;
mod handlers;
mod models;
mod db;
mod middleware;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .configure(routes::auth::auth_routes)
            // Add other configurations and middleware here
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}