use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::db;

#[derive(serde::Deserialize)]
pub struct TrendingQuery {
    days: Option<i32>,
    limit: Option<i32>,
}

pub async fn trending(
    pool: web::Data<PgPool>,
    query: web::Query<TrendingQuery>,
) -> HttpResponse {
    let days = query.days.unwrap_or(30);
    let limit = query.limit.unwrap_or(20);
    match db::trending_events(&pool, days, limit).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"error": e.to_string()})),
    }
}

pub async fn timeline(pool: web::Data<PgPool>) -> HttpResponse {
    match db::events_timeline(&pool).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"error": e.to_string()})),
    }
}
