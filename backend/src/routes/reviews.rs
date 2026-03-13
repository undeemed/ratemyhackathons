use actix_web::{get, post, web, HttpResponse};
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

use crate::errors::ApiError;
use crate::models::review::*;

/// Build a nested comment tree from flat rows
fn build_comment_tree(rows: Vec<ReviewCommentRow>) -> Vec<CommentNode> {
    // Create all nodes first
    let mut nodes: HashMap<Uuid, CommentNode> = HashMap::new();
    let mut child_ids: HashMap<Uuid, Vec<Uuid>> = HashMap::new(); // parent → children
    let mut root_ids: Vec<Uuid> = Vec::new();

    for row in &rows {
        nodes.insert(row.id, CommentNode {
            id: row.id,
            user_id: row.user_id,
            username: row.username.clone(),
            body: row.body.clone(),
            created_at: row.created_at,
            replies: vec![],
        });

        match row.parent_comment_id {
            Some(parent_id) => {
                child_ids.entry(parent_id).or_default().push(row.id);
            }
            None => {
                root_ids.push(row.id);
            }
        }
    }

    // Recursively attach children (bottom-up via post-order)
    fn attach_children(
        node_id: Uuid,
        nodes: &mut HashMap<Uuid, CommentNode>,
        child_ids: &HashMap<Uuid, Vec<Uuid>>,
    ) -> CommentNode {
        let children = child_ids.get(&node_id).cloned().unwrap_or_default();
        let replies: Vec<CommentNode> = children
            .into_iter()
            .map(|cid| attach_children(cid, nodes, child_ids))
            .collect();

        let mut node = nodes.remove(&node_id).unwrap();
        node.replies = replies;
        node
    }

    root_ids
        .into_iter()
        .map(|id| attach_children(id, &mut nodes, &child_ids))
        .collect()
}

/// GET /api/reviews/{id} — Get review detail with votes and threaded comments
#[get("/reviews/{id}")]
pub async fn get_review(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let review_id = id.into_inner();

    let review = sqlx::query_as::<_, Review>(
        "SELECT id, event_id, user_id, rating, title, body, created_at FROM reviews WHERE id = $1",
    )
    .bind(review_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("Review {} not found", review_id)))?;

    // Vote counts
    let helpful: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM review_votes WHERE review_id = $1 AND helpful = true",
    )
    .bind(review_id)
    .fetch_one(pool.get_ref())
    .await?;

    let unhelpful: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM review_votes WHERE review_id = $1 AND helpful = false",
    )
    .bind(review_id)
    .fetch_one(pool.get_ref())
    .await?;

    // All comments (flat, then tree-assembled)
    let comment_rows = sqlx::query_as::<_, ReviewCommentRow>(
        r#"
        SELECT rc.id, rc.user_id, u.username, rc.parent_comment_id, rc.body, rc.created_at
        FROM review_comments rc
        JOIN users u ON u.id = rc.user_id
        WHERE rc.review_id = $1
        ORDER BY rc.created_at ASC
        "#,
    )
    .bind(review_id)
    .fetch_all(pool.get_ref())
    .await?;

    let comments = build_comment_tree(comment_rows);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "id": review.id,
        "event_id": review.event_id,
        "user_id": review.user_id,
        "rating": review.rating,
        "title": review.title,
        "body": review.body,
        "created_at": review.created_at,
        "votes": {
            "helpful": helpful.0,
            "unhelpful": unhelpful.0
        },
        "comments": comments
    })))
}

/// POST /api/reviews/{id}/vote — Vote helpful or unhelpful
#[post("/reviews/{id}/vote")]
pub async fn vote_review(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
    body: web::Json<CreateReviewVote>,
) -> Result<HttpResponse, ApiError> {
    let review_id = id.into_inner();

    // Verify review exists
    let review_exists: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM reviews WHERE id = $1")
        .bind(review_id)
        .fetch_optional(pool.get_ref())
        .await?;

    if review_exists.is_none() {
        return Err(ApiError::NotFound(format!("Review {} not found", review_id)));
    }

    let vote_id = Uuid::now_v7();

    // Upsert: insert or update if user already voted
    let vote = sqlx::query_as::<_, ReviewVote>(
        r#"
        INSERT INTO review_votes (id, review_id, user_id, helpful)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (review_id, user_id)
        DO UPDATE SET helpful = EXCLUDED.helpful
        RETURNING id, review_id, user_id, helpful, created_at
        "#,
    )
    .bind(vote_id)
    .bind(review_id)
    .bind(body.user_id)
    .bind(body.helpful)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(vote))
}

/// POST /api/reviews/{id}/comments — Add a comment (top-level or reply)
#[post("/reviews/{id}/comments")]
pub async fn create_review_comment(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
    body: web::Json<CreateReviewComment>,
) -> Result<HttpResponse, ApiError> {
    body.validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let review_id = id.into_inner();

    // Verify review exists
    let review_exists: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM reviews WHERE id = $1")
        .bind(review_id)
        .fetch_optional(pool.get_ref())
        .await?;

    if review_exists.is_none() {
        return Err(ApiError::NotFound(format!("Review {} not found", review_id)));
    }

    // If replying, verify parent comment exists and belongs to this review
    if let Some(parent_id) = body.parent_comment_id {
        let parent_exists: Option<(Uuid,)> = sqlx::query_as(
            "SELECT id FROM review_comments WHERE id = $1 AND review_id = $2",
        )
        .bind(parent_id)
        .bind(review_id)
        .fetch_optional(pool.get_ref())
        .await?;

        if parent_exists.is_none() {
            return Err(ApiError::NotFound(format!("Parent comment {} not found", parent_id)));
        }
    }

    let comment_id = Uuid::now_v7();

    let comment = sqlx::query_as::<_, ReviewComment>(
        r#"
        INSERT INTO review_comments (id, review_id, user_id, parent_comment_id, body)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, review_id, user_id, parent_comment_id, body, created_at
        "#,
    )
    .bind(comment_id)
    .bind(review_id)
    .bind(body.user_id)
    .bind(body.parent_comment_id)
    .bind(ammonia::clean(&body.body))
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(comment))
}

/// GET /api/reviews/{id}/comments — Get threaded comments for a review
#[get("/reviews/{id}/comments")]
pub async fn list_review_comments(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let review_id = id.into_inner();

    let comment_rows = sqlx::query_as::<_, ReviewCommentRow>(
        r#"
        SELECT rc.id, rc.user_id, u.username, rc.parent_comment_id, rc.body, rc.created_at
        FROM review_comments rc
        JOIN users u ON u.id = rc.user_id
        WHERE rc.review_id = $1
        ORDER BY rc.created_at ASC
        "#,
    )
    .bind(review_id)
    .fetch_all(pool.get_ref())
    .await?;

    let comments = build_comment_tree(comment_rows);

    Ok(HttpResponse::Ok().json(comments))
}
