use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::db;

pub async fn stats(pool: web::Data<PgPool>) -> HttpResponse {
    match db::rating_distribution(&pool).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"error": e.to_string()})),
    }
}

#[derive(serde::Deserialize)]
pub struct RecentQuery {
    limit: Option<i32>,
}

pub async fn recent(
    pool: web::Data<PgPool>,
    query: web::Query<RecentQuery>,
) -> HttpResponse {
    let limit = query.limit.unwrap_or(20);
    match db::recent_reviews(&pool, limit).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"error": e.to_string()})),
    }
}
