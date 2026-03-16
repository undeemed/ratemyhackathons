use actix_web::{get, post, web, HttpRequest, HttpResponse};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::auth::AuthState;
use crate::errors::ApiError;
use crate::models::tag::*;

/// GET /api/tags — List all tags alphabetically
#[get("/tags")]
pub async fn list_tags(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiError> {
    let tags = sqlx::query_as::<_, Tag>(
        "SELECT id, name FROM tags ORDER BY name ASC",
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(tags))
}

#[derive(Debug, serde::Deserialize)]
pub struct TopTagsParams {
    pub entity_type: String,
    pub entity_id: Uuid,
}

/// GET /api/tags/top?entity_type=event&entity_id=uuid — Top 5 tags for an entity
#[get("/tags/top")]
pub async fn top_tags(
    pool: web::Data<PgPool>,
    query: web::Query<TopTagsParams>,
) -> Result<HttpResponse, ApiError> {
    let tags = match query.entity_type.as_str() {
        "event" => {
            sqlx::query_as::<_, TagCount>(
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
            .bind(query.entity_id)
            .fetch_all(pool.get_ref())
            .await?
        }
        "company" => {
            sqlx::query_as::<_, TagCount>(
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
            .bind(query.entity_id)
            .fetch_all(pool.get_ref())
            .await?
        }
        _ => {
            return Err(ApiError::BadRequest(
                "entity_type must be 'event' or 'company'".to_string(),
            ));
        }
    };

    Ok(HttpResponse::Ok().json(tags))
}

/// POST /api/tags — Create a new tag (returns existing if name matches)
#[post("/tags")]
pub async fn create_tag(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    auth_state: Option<web::Data<AuthState>>,
    body: web::Json<CreateTag>,
) -> Result<HttpResponse, ApiError> {
    crate::auth::require_auth_if_configured(&req, &auth_state, pool.get_ref()).await?;

    body.validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let name = ammonia::clean(&body.name).to_lowercase();

    // Return existing tag if name matches (case-insensitive)
    if let Some(existing) = sqlx::query_as::<_, Tag>(
        "SELECT id, name FROM tags WHERE LOWER(name) = $1",
    )
    .bind(&name)
    .fetch_optional(pool.get_ref())
    .await?
    {
        return Ok(HttpResponse::Ok().json(existing));
    }

    let id = Uuid::now_v7();
    let tag = sqlx::query_as::<_, Tag>(
        "INSERT INTO tags (id, name) VALUES ($1, $2) RETURNING id, name",
    )
    .bind(id)
    .bind(&name)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(tag))
}

#[derive(Debug, Serialize)]
struct VoteResponse {
    voted: bool,
    vote_count: i64,
}

/// POST /api/tags/{id}/vote — Toggle vote on a tag (upvote/un-upvote)
#[post("/tags/{id}/vote")]
pub async fn vote_tag(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    auth_state: Option<web::Data<AuthState>>,
    id: web::Path<Uuid>,
    body: web::Json<VoteTag>,
) -> Result<HttpResponse, ApiError> {
    let tag_id = id.into_inner();
    let user_id = crate::auth::resolve_user_id(&req, &auth_state, pool.get_ref(), body.user_id).await?;

    // Verify tag exists
    let tag_exists: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM tags WHERE id = $1")
        .bind(tag_id)
        .fetch_optional(pool.get_ref())
        .await?;

    if tag_exists.is_none() {
        return Err(ApiError::NotFound(format!("Tag {} not found", tag_id)));
    }

    // Toggle: delete if exists, insert if not
    let existing: Option<(Uuid,)> = sqlx::query_as(
        "SELECT id FROM tag_votes WHERE tag_id = $1 AND user_id = $2",
    )
    .bind(tag_id)
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let voted = if existing.is_some() {
        sqlx::query("DELETE FROM tag_votes WHERE tag_id = $1 AND user_id = $2")
            .bind(tag_id)
            .bind(user_id)
            .execute(pool.get_ref())
            .await?;
        false
    } else {
        let vote_id = Uuid::now_v7();
        sqlx::query(
            "INSERT INTO tag_votes (id, tag_id, user_id) VALUES ($1, $2, $3)",
        )
        .bind(vote_id)
        .bind(tag_id)
        .bind(user_id)
        .execute(pool.get_ref())
        .await?;
        true
    };

    let vote_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM tag_votes WHERE tag_id = $1",
    )
    .bind(tag_id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(VoteResponse {
        voted,
        vote_count: vote_count.0,
    }))
}
