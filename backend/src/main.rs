mod auth;
mod config;
mod db;
mod errors;
mod models;
mod routes;

use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{get, web, App, HttpResponse, HttpServer, middleware};
use actix_web::http::header;
use actix_web::error::{JsonPayloadError, QueryPayloadError};
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

/// Custom JSON error handler to prevent internal detail leakage
fn json_error_handler(
    err: JsonPayloadError,
    _req: &actix_web::HttpRequest,
) -> actix_web::Error {
    let detail = match &err {
        JsonPayloadError::ContentType => "Invalid content type".to_string(),
        JsonPayloadError::Payload(_) => "Payload error".to_string(),
        _ => "Invalid JSON input".to_string(),
    };
    let response = HttpResponse::BadRequest().json(serde_json::json!({
        "error": "bad_request",
        "message": detail
    }));
    actix_web::error::InternalError::from_response(err, response).into()
}

/// Custom query error handler
fn query_error_handler(
    err: QueryPayloadError,
    _req: &actix_web::HttpRequest,
) -> actix_web::Error {
    let response = HttpResponse::BadRequest().json(serde_json::json!({
        "error": "bad_request",
        "message": "Invalid query parameters"
    }));
    actix_web::error::InternalError::from_response(err, response).into()
}

/// Custom path error handler
fn path_error_handler(
    err: actix_web::error::PathError,
    _req: &actix_web::HttpRequest,
) -> actix_web::Error {
    let response = HttpResponse::BadRequest().json(serde_json::json!({
        "error": "bad_request",
        "message": "Invalid path parameter"
    }));
    actix_web::error::InternalError::from_response(err, response).into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let cfg = config::Config::from_env();
    log::info!("Starting server at {}:{}", cfg.host, cfg.port);

    let pool = db::init_pool(&cfg.database_url).await;
    log::info!("Database pool initialized");

    let bind_addr = (cfg.host.clone(), cfg.port);

    // Auth state for Clerk JWT verification (optional — runs without auth in dev if not configured)
    let auth_state = cfg.clerk_jwks_url.as_ref().map(|url| {
        log::info!("Clerk auth enabled (JWKS: {})", url);
        auth::AuthState::new(url.clone(), cfg.clerk_issuer.clone())
    });

    // Rate limiter: ~60 requests per minute per IP, burst of 30
    let governor_conf = GovernorConfigBuilder::default()
        .seconds_per_request(1)
        .burst_size(30)
        .finish()
        .unwrap();

    HttpServer::new(move || {
        // CORS: only allow the frontend origin (+ localhost for dev)
        let cors = Cors::default()
            .allowed_origin("https://ratemyhackathons.com")
            .allowed_origin("https://www.ratemyhackathons.com")
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .max_age(3600);

        let mut app = App::new()
            .wrap(Governor::new(&governor_conf))
            .wrap(cors)
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("Strict-Transport-Security", "max-age=31536000; includeSubDomains; preload"))
                    .add(("X-Frame-Options", "DENY"))
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add(("Referrer-Policy", "strict-origin-when-cross-origin"))
                    .add(("Permissions-Policy", "camera=(), microphone=(), geolocation=()"))
                    .add(("Content-Security-Policy", "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' https: data:; font-src 'self' https:; connect-src 'self'; frame-ancestors 'none'"))
            )
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::JsonConfig::default().limit(1024 * 1024).error_handler(json_error_handler))
            .app_data(web::QueryConfig::default().error_handler(query_error_handler))
            .app_data(web::PathConfig::default().error_handler(path_error_handler));

        // Register auth state if Clerk is configured
        if let Some(state) = auth_state.clone() {
            app = app.app_data(web::Data::new(state.clone()));
        }

        app.service(health_check)
            .service(
                web::scope("/api")
                    .service(routes::events::list_events)
                    .service(routes::events::globe_markers)
                    .service(routes::events::list_locations)
                    .service(routes::events::get_event)
                    .service(routes::events::create_event)
                    .service(routes::companies::list_companies)
                    .service(routes::companies::get_company)
                    .service(routes::companies::create_company)
                    .service(routes::users::list_users)
                    .service(routes::users::get_user)
                    .service(routes::users::create_user)
                    .service(routes::users::create_review)
                    .service(routes::reviews::get_review)
                    .service(routes::reviews::vote_review)
                    .service(routes::reviews::create_review_comment)
                    .service(routes::reviews::list_review_comments)
                    .service(routes::search::search)
                    .service(routes::tags::list_tags)
                    .service(routes::tags::top_tags)
                    .service(routes::tags::create_tag)
                    .service(routes::tags::vote_tag)
                    .service(routes::compare::compare)
            )
    })
    .bind(bind_addr)?
    .run()
    .await
}
