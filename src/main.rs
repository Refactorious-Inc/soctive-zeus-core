use actix_web::{App, HttpServer, web};
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;

mod models;
mod handlers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::auth::auth_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}