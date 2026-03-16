use actix_web::{get, post, web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::auth::AuthState;
use crate::errors::ApiError;
use crate::models::event::*;
use super::{PaginatedResponse, PaginationParams};

#[derive(Debug, serde::Deserialize)]
pub struct EventListParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub company_id: Option<Uuid>,
}

#[get("/events")]
pub async fn list_events(
    pool: web::Data<PgPool>,
    query: web::Query<EventListParams>,
) -> Result<HttpResponse, ApiError> {
    let pagination = PaginationParams {
        page: query.page.map(|p| p.max(1)),
        per_page: query.per_page.map(|p| p.clamp(1, 100)),
    };
    let limit = pagination.limit();
    let offset = pagination.offset();

    // Single query with aggregated companies and review stats (no N+1)
    let (rows, total) = if let Some(company_id) = query.company_id {
        let rows = sqlx::query_as::<_, EventRow>(
            r#"
            SELECT e.id, e.name, e.description, e.location, e.url,
                   e.start_date, e.end_date, e.image_url, e.latitude, e.longitude,
                   e.created_at, e.updated_at,
                   COALESCE(rs.avg_rating, NULL) as avg_rating,
                   COALESCE(rs.review_count, 0) as review_count
            FROM events e
            JOIN event_companies ec ON ec.event_id = e.id AND ec.company_id = $1
            LEFT JOIN LATERAL (
                SELECT AVG(rating)::float8 as avg_rating, COUNT(*) as review_count
                FROM reviews WHERE event_id = e.id
            ) rs ON true
            ORDER BY e.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(company_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool.get_ref())
        .await?;

        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM events e JOIN event_companies ec ON ec.event_id = e.id WHERE ec.company_id = $1",
        )
        .bind(company_id)
        .fetch_one(pool.get_ref())
        .await?;

        (rows, total.0)
    } else {
        let rows = sqlx::query_as::<_, EventRow>(
            r#"
            SELECT e.id, e.name, e.description, e.location, e.url,
                   e.start_date, e.end_date, e.image_url, e.latitude, e.longitude,
                   e.created_at, e.updated_at,
                   COALESCE(rs.avg_rating, NULL) as avg_rating,
                   COALESCE(rs.review_count, 0) as review_count
            FROM events e
            LEFT JOIN LATERAL (
                SELECT AVG(rating)::float8 as avg_rating, COUNT(*) as review_count
                FROM reviews WHERE event_id = e.id
            ) rs ON true
            ORDER BY e.created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool.get_ref())
        .await?;

        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM events")
            .fetch_one(pool.get_ref())
            .await?;

        (rows, total.0)
    };

    // Batch-fetch companies for all events in one query
    let event_ids: Vec<Uuid> = rows.iter().map(|r| r.id).collect();
    let company_rows = sqlx::query_as::<_, EventCompanyRow>(
        r#"
        SELECT ec.event_id, c.id, c.name, ec.role
        FROM companies c
        JOIN event_companies ec ON ec.company_id = c.id
        WHERE ec.event_id = ANY($1)
        "#,
    )
    .bind(&event_ids)
    .fetch_all(pool.get_ref())
    .await?;

    // Batch-fetch category ratings for all events in one query
    let cat_rows = sqlx::query_as::<_, (Uuid, String, f64)>(
        r#"
        SELECT r.event_id, rr.category, AVG(rr.score)::float8 as avg
        FROM review_ratings rr
        JOIN reviews r ON r.id = rr.review_id
        WHERE r.event_id = ANY($1)
        GROUP BY r.event_id, rr.category
        "#,
    )
    .bind(&event_ids)
    .fetch_all(pool.get_ref())
    .await?;

    let summaries: Vec<EventSummary> = rows
        .into_iter()
        .map(|row| {
            let companies: Vec<EventCompanyRef> = company_rows
                .iter()
                .filter(|c| c.event_id == row.id)
                .map(|c| EventCompanyRef {
                    id: c.id,
                    name: c.name.clone(),
                    role: c.role.clone(),
                })
                .collect();

            let category_ratings: Vec<crate::models::review::CategoryAvg> = cat_rows
                .iter()
                .filter(|(eid, _, _)| *eid == row.id)
                .map(|(_, cat, avg)| crate::models::review::CategoryAvg {
                    category: cat.clone(),
                    avg: *avg,
                })
                .collect();

            EventSummary {
                id: row.id,
                name: row.name,
                description: row.description,
                location: row.location,
                url: row.url,
                start_date: row.start_date,
                end_date: row.end_date,
                image_url: row.image_url,
                latitude: row.latitude,
                longitude: row.longitude,
                companies,
                avg_rating: row.avg_rating,
                review_count: row.review_count,
                category_ratings,
                created_at: row.created_at,
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(PaginatedResponse {
        data: summaries,
        total,
        page: pagination.page(),
        per_page: limit,
    }))
}

#[get("/events/{id}")]
pub async fn get_event(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let event_id = id.into_inner();

    let event = sqlx::query_as::<_, Event>(
        "SELECT id, name, description, location, url, start_date, end_date, image_url, latitude, longitude, created_at, updated_at FROM events WHERE id = $1",
    )
    .bind(event_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("Event {} not found", event_id)))?;

    let companies = sqlx::query_as::<_, EventCompanyRef>(
        r#"
        SELECT c.id, c.name, ec.role
        FROM companies c
        JOIN event_companies ec ON ec.company_id = c.id
        WHERE ec.event_id = $1
        "#,
    )
    .bind(event_id)
    .fetch_all(pool.get_ref())
    .await?;

    let review_rows = sqlx::query_as::<_, EventReviewRow>(
        r#"
        SELECT r.id, r.user_id, u.username, r.rating, r.title, r.body, r.would_return, r.created_at
        FROM reviews r
        JOIN users u ON u.id = r.user_id
        WHERE r.event_id = $1
        ORDER BY r.created_at DESC
        "#,
    )
    .bind(event_id)
    .fetch_all(pool.get_ref())
    .await?;

    let stats: (Option<f64>, i64) = sqlx::query_as(
        "SELECT AVG(rating)::float8, COUNT(*) FROM reviews WHERE event_id = $1",
    )
    .bind(event_id)
    .fetch_one(pool.get_ref())
    .await?;

    // Would return percentage
    let would_return: (Option<f64>,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FILTER (WHERE would_return = true) * 100.0 / NULLIF(COUNT(*), 0)
        FROM reviews WHERE event_id = $1
        "#,
    )
    .bind(event_id)
    .fetch_one(pool.get_ref())
    .await?;

    // Category averages
    let category_ratings = sqlx::query_as::<_, crate::models::review::CategoryAvg>(
        r#"
        SELECT rr.category, AVG(rr.score)::float8 as avg
        FROM review_ratings rr
        JOIN reviews r ON r.id = rr.review_id
        WHERE r.event_id = $1
        GROUP BY rr.category
        "#,
    )
    .bind(event_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Top tags
    let top_tags = sqlx::query_as::<_, crate::models::tag::TagCount>(
        r#"
        SELECT t.name, COUNT(*) as count
        FROM review_tags rt
        JOIN tags t ON t.id = rt.tag_id
        JOIN reviews r ON r.id = rt.review_id
        WHERE r.event_id = $1
        GROUP BY t.name
        ORDER BY count DESC
        LIMIT 5
        "#,
    )
    .bind(event_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Rating distribution
    let rating_distribution = sqlx::query_as::<_, crate::models::review::RatingDistribution>(
        r#"
        SELECT rating, COUNT(*) as count
        FROM reviews WHERE event_id = $1
        GROUP BY rating ORDER BY rating DESC
        "#,
    )
    .bind(event_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Sponsors
    let sponsors = sqlx::query_as::<_, crate::models::sponsor::EventSponsor>(
        "SELECT id, name, logo_url FROM event_sponsors WHERE event_id = $1 ORDER BY name",
    )
    .bind(event_id)
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

    let reviews: Vec<EventReviewRef> = review_rows
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
            EventReviewRef {
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

    Ok(HttpResponse::Ok().json(EventDetail {
        id: event.id,
        name: event.name,
        description: event.description,
        location: event.location,
        url: event.url,
        start_date: event.start_date,
        end_date: event.end_date,
        image_url: event.image_url,
        latitude: event.latitude,
        longitude: event.longitude,
        companies,
        reviews,
        avg_rating: stats.0,
        review_count: stats.1,
        would_return_pct: would_return.0,
        category_ratings,
        top_tags,
        rating_distribution,
        sponsors,
    }))
}

#[post("/events")]
pub async fn create_event(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    auth_state: Option<web::Data<AuthState>>,
    body: web::Json<CreateEvent>,
) -> Result<HttpResponse, ApiError> {
    crate::auth::require_auth_if_configured(&req, &auth_state, pool.get_ref()).await?;

    body.validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let event_id = Uuid::now_v7();

    // Sanitize free-text fields
    let name = ammonia::clean(&body.name);
    let description = body.description.as_deref().map(|s| ammonia::clean(s));
    let location = body.location.as_deref().map(|s| ammonia::clean(s));

    // Use a transaction so event + company attachments are atomic
    let mut tx = pool.begin().await?;

    sqlx::query(
        r#"
        INSERT INTO events (id, name, description, location, url, start_date, end_date, image_url, latitude, longitude)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        "#,
    )
    .bind(event_id)
    .bind(&name)
    .bind(&description)
    .bind(&location)
    .bind(&body.url)
    .bind(body.start_date)
    .bind(body.end_date)
    .bind(&body.image_url)
    .bind(body.latitude)
    .bind(body.longitude)
    .execute(&mut *tx)
    .await?;

    // Attach companies if provided
    if let Some(ref company_ids) = body.company_ids {
        for cid in company_ids {
            sqlx::query(
                "INSERT INTO event_companies (event_id, company_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            )
            .bind(event_id)
            .bind(cid)
            .execute(&mut *tx)
            .await?;
        }
    }

    tx.commit().await?;

    let event = sqlx::query_as::<_, Event>(
        "SELECT id, name, description, location, url, start_date, end_date, image_url, latitude, longitude, created_at, updated_at FROM events WHERE id = $1",
    )
    .bind(event_id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(event))
}

#[get("/events/globe")]
pub async fn globe_markers(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiError> {
    let markers = sqlx::query_as::<_, GlobeMarker>(
        r#"
        SELECT id, name, latitude, longitude, start_date
        FROM events
        WHERE latitude IS NOT NULL AND longitude IS NOT NULL
        "#,
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(markers))
}

#[get("/events/locations")]
pub async fn list_locations(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiError> {
    let rows = sqlx::query_scalar::<_, String>(
        "SELECT DISTINCT location FROM events WHERE location IS NOT NULL ORDER BY location",
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(rows))
}
