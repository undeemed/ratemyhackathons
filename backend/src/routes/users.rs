use actix_web::{get, post, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::errors::ApiError;
use crate::models::user::*;
use crate::models::review::*;
use super::{PaginatedResponse, PaginationParams};

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
        SELECT r.id, r.event_id, e.name as event_name, r.rating, r.title, r.body, r.created_at
        FROM reviews r
        JOIN events e ON e.id = r.event_id
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
            rating: r.rating,
            title: r.title,
            body: r.body,
            created_at: r.created_at,
        }).collect(),
    }))
}

#[post("/users")]
pub async fn create_user(
    pool: web::Data<PgPool>,
    body: web::Json<CreateUser>,
) -> Result<HttpResponse, ApiError> {
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

#[post("/reviews")]
pub async fn create_review(
    pool: web::Data<PgPool>,
    body: web::Json<CreateReview>,
) -> Result<HttpResponse, ApiError> {
    body.validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    // Verify event and user exist in a single query
    let exists: Option<(Uuid, Uuid)> = sqlx::query_as(
        r#"
        SELECT e.id, u.id
        FROM events e, users u
        WHERE e.id = $1 AND u.id = $2
        "#,
    )
    .bind(body.event_id)
    .bind(body.user_id)
    .fetch_optional(pool.get_ref())
    .await?;

    match exists {
        None => {
            let event_exists: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM events WHERE id = $1")
                .bind(body.event_id)
                .fetch_optional(pool.get_ref())
                .await?;

            if event_exists.is_none() {
                return Err(ApiError::NotFound(format!("Event {} not found", body.event_id)));
            }
            return Err(ApiError::NotFound(format!("User {} not found", body.user_id)));
        }
        Some(_) => {}
    }

    let id = Uuid::now_v7();

    // Sanitize free-text fields
    let title = body.title.as_deref().map(|s| ammonia::clean(s));
    let review_body = body.body.as_deref().map(|s| ammonia::clean(s));

    let review = sqlx::query_as::<_, Review>(
        r#"
        INSERT INTO reviews (id, event_id, user_id, rating, title, body)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, event_id, user_id, rating, title, body, created_at
        "#,
    )
    .bind(id)
    .bind(body.event_id)
    .bind(body.user_id)
    .bind(body.rating)
    .bind(&title)
    .bind(&review_body)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(review))
}
