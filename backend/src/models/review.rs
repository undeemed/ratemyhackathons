use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

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

/// Aggregated vote counts for a review
#[derive(Debug, Serialize)]
pub struct ReviewVoteSummary {
    pub helpful: i64,
    pub unhelpful: i64,
}

// ── API requests ──

#[derive(Debug, Deserialize)]
pub struct CreateReview {
    pub event_id: Uuid,
    pub user_id: Uuid,
    pub rating: i32,
    pub title: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReviewVote {
    pub user_id: Uuid,
    pub helpful: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateReviewComment {
    pub user_id: Uuid,
    pub parent_comment_id: Option<Uuid>,  // None = top-level, Some = reply
    pub body: String,
}
