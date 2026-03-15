use actix_web::{get, post, web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::auth::AuthState;
use crate::errors::ApiError;
use crate::models::company::*;
use super::{PaginatedResponse, PaginationParams};

#[get("/companies")]
pub async fn list_companies(
    pool: web::Data<PgPool>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, ApiError> {
    let limit = query.limit();
    let offset = query.offset();

    // Single query with event count subquery (no N+1)
    let summaries = sqlx::query_as::<_, CompanySummary>(
        r#"
        SELECT c.id, c.name, c.logo_url, c.website, c.description,
               (SELECT COUNT(*) FROM event_companies WHERE company_id = c.id) as event_count,
               c.created_at
        FROM companies c
        ORDER BY c.name ASC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM companies")
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(PaginatedResponse {
        data: summaries,
        total: total.0,
        page: query.page(),
        per_page: limit,
    }))
}

#[get("/companies/{id}")]
pub async fn get_company(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let company_id = id.into_inner();

    let company = sqlx::query_as::<_, Company>(
        "SELECT id, name, logo_url, website, description, created_at FROM companies WHERE id = $1",
    )
    .bind(company_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("Company {} not found", company_id)))?;

    let events = sqlx::query_as::<_, CompanyEventRef>(
        r#"
        SELECT e.id, e.name, ec.role, e.start_date,
               (SELECT AVG(rating)::float8 FROM reviews WHERE event_id = e.id) as avg_rating
        FROM events e
        JOIN event_companies ec ON ec.event_id = e.id
        WHERE ec.company_id = $1
        ORDER BY e.start_date DESC NULLS LAST
        "#,
    )
    .bind(company_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Aggregate stats
    let stats: (Option<f64>, i64) = sqlx::query_as(
        "SELECT AVG(rating)::float8, COUNT(*) FROM reviews WHERE company_id = $1",
    )
    .bind(company_id)
    .fetch_one(pool.get_ref())
    .await?;

    // Would return percentage
    let would_return: (Option<f64>,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FILTER (WHERE would_return = true) * 100.0 / NULLIF(COUNT(*), 0)
        FROM reviews WHERE company_id = $1
        "#,
    )
    .bind(company_id)
    .fetch_one(pool.get_ref())
    .await?;

    // Category averages
    let category_ratings = sqlx::query_as::<_, crate::models::review::CategoryAvg>(
        r#"
        SELECT rr.category, AVG(rr.score)::float8 as avg
        FROM review_ratings rr
        JOIN reviews r ON r.id = rr.review_id
        WHERE r.company_id = $1
        GROUP BY rr.category
        "#,
    )
    .bind(company_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Top tags
    let top_tags = sqlx::query_as::<_, crate::models::tag::TagCount>(
        r#"
        SELECT t.name, COUNT(*) as count
        FROM review_tags rt
        JOIN tags t ON t.id = rt.tag_id
        JOIN reviews r ON r.id = rt.review_id
        WHERE r.company_id = $1
        GROUP BY t.name
        ORDER BY count DESC
        LIMIT 5
        "#,
    )
    .bind(company_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Rating distribution
    let rating_distribution = sqlx::query_as::<_, crate::models::review::RatingDistribution>(
        r#"
        SELECT rating, COUNT(*) as count
        FROM reviews WHERE company_id = $1
        GROUP BY rating ORDER BY rating DESC
        "#,
    )
    .bind(company_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Reviews with per-review category ratings
    let review_rows = sqlx::query_as::<_, CompanyReviewRow>(
        r#"
        SELECT r.id, r.user_id, u.username, r.rating, r.title, r.body, r.would_return, r.created_at
        FROM reviews r
        JOIN users u ON u.id = r.user_id
        WHERE r.company_id = $1
        ORDER BY r.created_at DESC
        "#,
    )
    .bind(company_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Batch-fetch category ratings for all reviews
    let review_ids: Vec<Uuid> = review_rows.iter().map(|r| r.id).collect();
    let all_ratings = sqlx::query_as::<_, (Uuid, String, i16)>(
        "SELECT review_id, category, score FROM review_ratings WHERE review_id = ANY($1)",
    )
    .bind(&review_ids)
    .fetch_all(pool.get_ref())
    .await?;

    let reviews: Vec<CompanyReviewRef> = review_rows
        .into_iter()
        .map(|r| {
            let cats: Vec<crate::models::review::ReviewRatingRow> = all_ratings
                .iter()
                .filter(|(rid, _, _)| *rid == r.id)
                .map(|(_, cat, score)| crate::models::review::ReviewRatingRow {
                    category: cat.clone(),
                    score: *score,
                })
                .collect();
            CompanyReviewRef {
                id: r.id,
                user_id: r.user_id,
                username: r.username,
                rating: r.rating,
                title: r.title,
                body: r.body,
                would_return: r.would_return,
                created_at: r.created_at,
                category_ratings: cats,
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(CompanyDetail {
        id: company.id,
        name: company.name,
        logo_url: company.logo_url,
        website: company.website,
        description: company.description,
        events,
        avg_rating: stats.0,
        review_count: stats.1,
        would_return_pct: would_return.0,
        category_ratings,
        top_tags,
        rating_distribution,
        reviews,
    }))
}

#[post("/companies")]
pub async fn create_company(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    auth_state: Option<web::Data<AuthState>>,
    body: web::Json<CreateCompany>,
) -> Result<HttpResponse, ApiError> {
    crate::auth::require_auth_if_configured(&req, &auth_state, pool.get_ref()).await?;

    body.validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let id = Uuid::now_v7();

    // Sanitize free-text fields
    let name = ammonia::clean(&body.name);
    let description = body.description.as_deref().map(|s| ammonia::clean(s));

    let company = sqlx::query_as::<_, Company>(
        r#"
        INSERT INTO companies (id, name, logo_url, website, description)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, logo_url, website, description, created_at
        "#,
    )
    .bind(id)
    .bind(&name)
    .bind(&body.logo_url)
    .bind(&body.website)
    .bind(&description)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(company))
}
