use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

// ── Database rows ──

#[derive(Debug, FromRow, Serialize)]
pub struct Review {
    pub id: Uuid,
    pub event_id: Uuid,
    pub user_id: Uuid,
    pub rating: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct ReviewVote {
    pub id: Uuid,
    pub review_id: Uuid,
    pub user_id: Uuid,
    pub helpful: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct ReviewComment {
    pub id: Uuid,
    pub review_id: Uuid,
    pub user_id: Uuid,
    pub parent_comment_id: Option<Uuid>,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

// ── API responses ──

/// Flat comment row from DB (assembled into tree in Rust)
#[derive(Debug, FromRow)]
pub struct ReviewCommentRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub parent_comment_id: Option<Uuid>,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

/// Nested comment tree node (Reddit-style)
#[derive(Debug, Serialize, Clone)]
pub struct CommentNode {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub replies: Vec<CommentNode>,
}

// ── API requests ──

#[derive(Debug, Deserialize, Validate)]
pub struct CreateReview {
    pub event_id: Uuid,
    pub user_id: Uuid,

    #[validate(range(min = 1, max = 5, message = "Rating must be between 1 and 5"))]
    pub rating: i32,

    #[validate(length(max = 200, message = "Review title must be under 200 characters"))]
    pub title: Option<String>,

    #[validate(length(max = 5000, message = "Review body must be under 5000 characters"))]
    pub body: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateReviewVote {
    pub user_id: Uuid,
    pub helpful: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateReviewComment {
    pub user_id: Uuid,
    pub parent_comment_id: Option<Uuid>,

    #[validate(length(min = 1, max = 2000, message = "Comment must be 1-2000 characters"))]
    pub body: String,
}
