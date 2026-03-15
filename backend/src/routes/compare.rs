use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::models::review::CategoryAvg;

#[derive(Debug, serde::Deserialize)]
pub struct CompareParams {
    #[serde(rename = "type")]
    pub entity_type: String,
    pub ids: String,
}

#[derive(Debug, serde::Serialize)]
pub struct CompareEntity {
    pub id: Uuid,
    pub name: String,
    pub avg_rating: Option<f64>,
    pub review_count: i64,
    pub would_return_pct: Option<f64>,
    pub category_ratings: Vec<CategoryAvg>,
}

/// GET /api/compare?type=company&ids=uuid1,uuid2
#[get("/compare")]
pub async fn compare(
    pool: web::Data<PgPool>,
    query: web::Query<CompareParams>,
) -> Result<HttpResponse, ApiError> {
    let ids: Vec<Uuid> = query
        .ids
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    if ids.len() < 2 || ids.len() > 4 {
        return Err(ApiError::BadRequest(
            "Provide 2-4 comma-separated UUIDs".to_string(),
        ));
    }

    let mut entities = Vec::new();

    for entity_id in &ids {
        let (name, stats, would_return, category_ratings) = match query.entity_type.as_str() {
            "company" => {
                let name: Option<(String,)> =
                    sqlx::query_as("SELECT name FROM companies WHERE id = $1")
                        .bind(entity_id)
                        .fetch_optional(pool.get_ref())
                        .await?;
                let name = name
                    .ok_or_else(|| ApiError::NotFound(format!("Company {} not found", entity_id)))?
                    .0;

                let stats: (Option<f64>, i64) = sqlx::query_as(
                    "SELECT AVG(rating)::float8, COUNT(*) FROM reviews WHERE company_id = $1",
                )
                .bind(entity_id)
                .fetch_one(pool.get_ref())
                .await?;

                let wr: (Option<f64>,) = sqlx::query_as(
                    "SELECT COUNT(*) FILTER (WHERE would_return = true) * 100.0 / NULLIF(COUNT(*), 0) FROM reviews WHERE company_id = $1",
                )
                .bind(entity_id)
                .fetch_one(pool.get_ref())
                .await?;

                let cats = sqlx::query_as::<_, CategoryAvg>(
                    r#"
                    SELECT rr.category, AVG(rr.score)::float8 as avg
                    FROM review_ratings rr
                    JOIN reviews r ON r.id = rr.review_id
                    WHERE r.company_id = $1
                    GROUP BY rr.category
                    "#,
                )
                .bind(entity_id)
                .fetch_all(pool.get_ref())
                .await?;

                (name, stats, wr, cats)
            }
            "event" => {
                let name: Option<(String,)> =
                    sqlx::query_as("SELECT name FROM events WHERE id = $1")
                        .bind(entity_id)
                        .fetch_optional(pool.get_ref())
                        .await?;
                let name = name
                    .ok_or_else(|| ApiError::NotFound(format!("Event {} not found", entity_id)))?
                    .0;

                let stats: (Option<f64>, i64) = sqlx::query_as(
                    "SELECT AVG(rating)::float8, COUNT(*) FROM reviews WHERE event_id = $1",
                )
                .bind(entity_id)
                .fetch_one(pool.get_ref())
                .await?;

                let wr: (Option<f64>,) = sqlx::query_as(
                    "SELECT COUNT(*) FILTER (WHERE would_return = true) * 100.0 / NULLIF(COUNT(*), 0) FROM reviews WHERE event_id = $1",
                )
                .bind(entity_id)
                .fetch_one(pool.get_ref())
                .await?;

                let cats = sqlx::query_as::<_, CategoryAvg>(
                    r#"
                    SELECT rr.category, AVG(rr.score)::float8 as avg
                    FROM review_ratings rr
                    JOIN reviews r ON r.id = rr.review_id
                    WHERE r.event_id = $1
                    GROUP BY rr.category
                    "#,
                )
                .bind(entity_id)
                .fetch_all(pool.get_ref())
                .await?;

                (name, stats, wr, cats)
            }
            _ => {
                return Err(ApiError::BadRequest(
                    "type must be 'event' or 'company'".to_string(),
                ));
            }
        };

        entities.push(CompareEntity {
            id: *entity_id,
            name,
            avg_rating: stats.0,
            review_count: stats.1,
            would_return_pct: would_return.0,
            category_ratings,
        });
    }

    Ok(HttpResponse::Ok().json(entities))
}
