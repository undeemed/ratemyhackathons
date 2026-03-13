mod config;
mod db;
mod errors;
mod models;
mod routes;

use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, middleware};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
}

#[get("/health")]
async fn health_check(pool: web::Data<PgPool>) -> HttpResponse {
    // Verify DB connectivity
    match sqlx::query("SELECT 1").execute(pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(HealthResponse {
            status: "healthy",
            version: env!("CARGO_PKG_VERSION"),
        }),
        Err(_) => HttpResponse::ServiceUnavailable().json(HealthResponse {
            status: "unhealthy",
            version: env!("CARGO_PKG_VERSION"),
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let cfg = config::Config::from_env();
    log::info!("Starting server at {}:{}", cfg.host, cfg.port);

    let pool = db::init_pool(&cfg.database_url).await;
    log::info!("Database pool initialized");

    let bind_addr = (cfg.host.clone(), cfg.port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::JsonConfig::default().limit(1024 * 1024)) // 1MB JSON limit
            .service(health_check)
            .service(
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
                    .service(routes::search::search)
            )
    })
    .bind(bind_addr)?
    .run()
    .await
}
