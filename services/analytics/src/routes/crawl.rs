use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::db;

pub async fn stats(pool: web::Data<PgPool>) -> HttpResponse {
    match db::crawl_stats(&pool).await {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"error": e.to_string()})),
    }
}

#[derive(serde::Deserialize)]
pub struct HistoryQuery {
    days: Option<i32>,
}

pub async fn history(
    pool: web::Data<PgPool>,
    query: web::Query<HistoryQuery>,
) -> HttpResponse {
    let days = query.days.unwrap_or(30);
    match db::crawl_history(&pool, days).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"error": e.to_string()})),
    }
}

pub async fn sources(pool: web::Data<PgPool>) -> HttpResponse {
    match db::crawl_sources(&pool).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"error": e.to_string()})),
    }
}
