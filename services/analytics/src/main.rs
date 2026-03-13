use actix_cors::Cors;
use actix_web::{web, App, HttpServer, middleware};
use sqlx::postgres::PgPoolOptions;
use std::env;

mod db;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let bind = env::var("ANALYTICS_BIND").unwrap_or_else(|_| "127.0.0.1:8081".to_string());
    println!("Analytics server running at http://{bind}");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::configure)
    })
    .bind(&bind)?
    .run()
    .await
}
