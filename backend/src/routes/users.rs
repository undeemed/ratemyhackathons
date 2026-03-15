use actix_web::{get, post, web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::auth::AuthState;
use crate::errors::ApiError;
use crate::models::user::*;
use crate::models::review::*;
use super::{PaginatedResponse, PaginationParams};

/// Resolve user_id from JWT auth (if configured) or from request body fallback (dev mode)
async fn resolve_user_id(
    req: &HttpRequest,
    auth_state: &Option<web::Data<AuthState>>,
    pool: &PgPool,
    body_user_id: Option<Uuid>,
) -> Result<Uuid, ApiError> {
    if let Some(state) = auth_state {
        let auth_user = crate::auth::require_auth(req, state, pool).await
            .map_err(|e| ApiError::Unauthorized(e.to_string()))?;
        Ok(auth_user.user_id)
    } else {
        // Dev mode: require user_id in body
        body_user_id.ok_or_else(|| {
            ApiError::BadRequest("user_id is required (no auth configured)".to_string())
        })
    }
}

#[get("/users")]
pub async fn list_users(
    pool: web::Data<PgPool>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, ApiError> {
    let limit = query.limit();
    let offset = query.offset();

    let summaries = sqlx::query_as::<_, UserSummary>(
        r#"
        SELECT u.id, u.username, u.display_name, u.avatar_url,
               (SELECT COUNT(*) FROM reviews WHERE user_id = u.id) as review_count,
               u.created_at
        FROM users u
        ORDER BY u.username ASC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(PaginatedResponse {
        data: summaries,
        total: total.0,
        page: query.page(),
        per_page: limit,
    }))
}

#[get("/users/{id}")]
pub async fn get_user(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let user_id = id.into_inner();

    let user = sqlx::query_as::<_, User>(
        r#"SELECT id, username, email, display_name, bio, age, avatar_url,
                  github, twitter, linkedin, website, created_at
           FROM users WHERE id = $1"#,
    )
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("User {} not found", user_id)))?;

    let reviews = sqlx::query_as::<_, UserReviewRow>(
        r#"
        SELECT r.id, r.event_id, e.name as event_name,
               r.company_id, c.name as company_name,
               r.rating, r.title, r.body, r.created_at
        FROM reviews r
        LEFT JOIN events e ON e.id = r.event_id
        LEFT JOIN companies c ON c.id = r.company_id
        WHERE r.user_id = $1
        ORDER BY r.created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(UserDetail {
        id: user.id,
        username: user.username,
        email: user.email,
        display_name: user.display_name,
        bio: user.bio,
        age: user.age,
        avatar_url: user.avatar_url,
        socials: UserSocials {
            github: user.github,
            twitter: user.twitter,
            linkedin: user.linkedin,
            website: user.website,
        },
        reviews: reviews.into_iter().map(|r| UserReviewRef {
            id: r.id,
            event_id: r.event_id,
            event_name: r.event_name,
            company_id: r.company_id,
            company_name: r.company_name,
            rating: r.rating,
            title: r.title,
            body: r.body,
            created_at: r.created_at,
        }).collect(),
    }))
}

#[post("/users")]
pub async fn create_user(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    auth_state: Option<web::Data<AuthState>>,
    body: web::Json<CreateUser>,
) -> Result<HttpResponse, ApiError> {
    crate::auth::require_auth_if_configured(&req, &auth_state, pool.get_ref()).await?;

    body.validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let id = Uuid::now_v7();

    // Sanitize free-text fields with ammonia
    let display_name = body.display_name.as_deref().map(|s| ammonia::clean(s));
    let bio = body.bio.as_deref().map(|s| ammonia::clean(s));

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, username, email, display_name, bio, age, avatar_url, github, twitter, linkedin, website)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING id, username, email, display_name, bio, age, avatar_url, github, twitter, linkedin, website, created_at
        "#,
    )
    .bind(id)
    .bind(ammonia::clean(&body.username))
    .bind(&body.email)
    .bind(&display_name)
    .bind(&bio)
    .bind(body.age)
    .bind(&body.avatar_url)
    .bind(&body.github)
    .bind(&body.twitter)
    .bind(&body.linkedin)
    .bind(&body.website)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(user))
}

const VALID_CATEGORIES: &[&str] = &[
    "organization", "prizes", "mentorship", "judging", "venue",
    "food", "swag", "networking", "communication", "vibes",
];

#[post("/reviews")]
pub async fn create_review(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    auth_state: Option<web::Data<AuthState>>,
    body: web::Json<CreateReview>,
) -> Result<HttpResponse, ApiError> {
    body.validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    // Resolve user from JWT or body
    let user_id = resolve_user_id(&req, &auth_state, pool.get_ref(), body.user_id).await?;

    // Validate XOR: exactly one target
    match (&body.event_id, &body.company_id) {
        (Some(_), None) | (None, Some(_)) => {}
        _ => {
            return Err(ApiError::BadRequest(
                "Exactly one of event_id or company_id must be provided".to_string(),
            ));
        }
    }

    // Validate all 10 categories present and scores in range
    if body.category_ratings.len() != 10 {
        return Err(ApiError::BadRequest(
            "All 10 category ratings are required".to_string(),
        ));
    }
    for cat in VALID_CATEGORIES {
        match body.category_ratings.get(*cat) {
            Some(&score) if (1..=5).contains(&score) => {}
            Some(_) => {
                return Err(ApiError::BadRequest(
                    format!("Category '{}' score must be between 1 and 5", cat),
                ));
            }
            None => {
                return Err(ApiError::BadRequest(
                    format!("Missing required category: {}", cat),
                ));
            }
        }
    }

    // Compute overall rating as average of category scores
    let total: i16 = body.category_ratings.values().sum();
    let rating = ((total as f64 / 10.0).round()) as i32;

    // Verify target entity exists
    if let Some(event_id) = body.event_id {
        let exists: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM events WHERE id = $1")
            .bind(event_id)
            .fetch_optional(pool.get_ref())
            .await?;
        if exists.is_none() {
            return Err(ApiError::NotFound(format!("Event {} not found", event_id)));
        }
    }
    if let Some(company_id) = body.company_id {
        let exists: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM companies WHERE id = $1")
            .bind(company_id)
            .fetch_optional(pool.get_ref())
            .await?;
        if exists.is_none() {
            return Err(ApiError::NotFound(format!("Company {} not found", company_id)));
        }
    }

    let id = Uuid::now_v7();

    // Sanitize free-text fields
    let title = body.title.as_deref().map(|s| ammonia::clean(s));
    let review_body = ammonia::clean(&body.body);

    // Transaction: insert review + 10 category ratings + tag links
    let mut tx = pool.begin().await?;

    sqlx::query(
        r#"
        INSERT INTO reviews (id, event_id, company_id, user_id, rating, title, body, would_return)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(id)
    .bind(body.event_id)
    .bind(body.company_id)
    .bind(user_id)
    .bind(rating)
    .bind(&title)
    .bind(&review_body)
    .bind(body.would_return)
    .execute(&mut *tx)
    .await?;

    // Batch-insert category ratings
    for (category, score) in &body.category_ratings {
        sqlx::query(
            "INSERT INTO review_ratings (review_id, category, score) VALUES ($1, $2, $3)",
        )
        .bind(id)
        .bind(category)
        .bind(score)
        .execute(&mut *tx)
        .await?;
    }

    // Link tags if provided
    if let Some(ref tag_ids) = body.tag_ids {
        for tag_id in tag_ids {
            sqlx::query(
                "INSERT INTO review_tags (review_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            )
            .bind(id)
            .bind(tag_id)
            .execute(&mut *tx)
            .await?;
        }
    }

    tx.commit().await?;

    // Fetch the created review
    let review = sqlx::query_as::<_, Review>(
        "SELECT id, event_id, company_id, user_id, rating, title, body, would_return, created_at FROM reviews WHERE id = $1",
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(review))
}
