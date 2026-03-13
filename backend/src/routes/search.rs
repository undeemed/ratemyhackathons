use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiError;

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub q: String,
    #[serde(rename = "type")]
    pub search_type: Option<String>, // "event", "company", "user" (pipe-separated)
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub events: Vec<SearchResult>,
    pub companies: Vec<SearchResult>,
    pub users: Vec<SearchResult>,
    pub total: i64,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SearchResult {
    pub id: Uuid,
    pub name: String,
    pub rank: f32,
}

#[get("/search")]
pub async fn search(
    pool: web::Data<PgPool>,
    query: web::Query<SearchParams>,
) -> Result<HttpResponse, ApiError> {
    if query.q.trim().is_empty() {
        return Err(ApiError::BadRequest("Search query 'q' is required".to_string()));
    }

    let search_term = &query.q;
    let limit = query.per_page.unwrap_or(20).min(100);

    let search_types: Vec<&str> = match &query.search_type {
        Some(t) => t.split('|').collect(),
        None => vec!["event", "company", "user"],
    };

    let mut events = Vec::new();
    let mut companies = Vec::new();
    let mut users = Vec::new();

    if search_types.contains(&"event") {
        events = sqlx::query_as::<_, SearchResult>(
            r#"
            SELECT id, name, ts_rank(search_vector, plainto_tsquery('english', $1)) as rank
            FROM events
            WHERE search_vector @@ plainto_tsquery('english', $1)
            ORDER BY rank DESC
            LIMIT $2
            "#,
        )
        .bind(search_term)
        .bind(limit)
        .fetch_all(pool.get_ref())
        .await?;
    }

    if search_types.contains(&"company") {
        companies = sqlx::query_as::<_, SearchResult>(
            r#"
            SELECT id, name, ts_rank(search_vector, plainto_tsquery('english', $1)) as rank
            FROM companies
            WHERE search_vector @@ plainto_tsquery('english', $1)
            ORDER BY rank DESC
            LIMIT $2
            "#,
        )
        .bind(search_term)
        .bind(limit)
        .fetch_all(pool.get_ref())
        .await?;
    }

    if search_types.contains(&"user") {
        users = sqlx::query_as::<_, SearchResult>(
            r#"
            SELECT id, username as name,
                   ts_rank(to_tsvector('english', username), plainto_tsquery('english', $1)) as rank
            FROM users
            WHERE to_tsvector('english', username) @@ plainto_tsquery('english', $1)
            ORDER BY rank DESC
            LIMIT $2
            "#,
        )
        .bind(search_term)
        .bind(limit)
        .fetch_all(pool.get_ref())
        .await?;
    }

    let total = (events.len() + companies.len() + users.len()) as i64;

    Ok(HttpResponse::Ok().json(SearchResponse {
        events,
        companies,
        users,
        total,
    }))
}
