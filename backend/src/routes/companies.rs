use actix_web::{get, post, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

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

    Ok(HttpResponse::Ok().json(CompanyDetail {
        id: company.id,
        name: company.name,
        logo_url: company.logo_url,
        website: company.website,
        description: company.description,
        events,
    }))
}

#[post("/companies")]
pub async fn create_company(
    pool: web::Data<PgPool>,
    body: web::Json<CreateCompany>,
) -> Result<HttpResponse, ApiError> {
    let id = Uuid::now_v7();

    let company = sqlx::query_as::<_, Company>(
        r#"
        INSERT INTO companies (id, name, logo_url, website, description)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, logo_url, website, description, created_at
        "#,
    )
    .bind(id)
    .bind(&body.name)
    .bind(&body.logo_url)
    .bind(&body.website)
    .bind(&body.description)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(company))
}
