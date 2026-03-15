use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

// ── Database rows ──

#[derive(Debug, FromRow, Serialize)]
pub struct Review {
    pub id: Uuid,
    pub event_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
    pub user_id: Uuid,
    pub rating: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub would_return: Option<bool>,
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

/// Per-category rating row from review_ratings table
#[derive(Debug, FromRow, Serialize)]
pub struct ReviewRatingRow {
    pub category: String,
    pub score: i16,
}

/// Category average for aggregated display
#[derive(Debug, FromRow, Serialize, Clone)]
pub struct CategoryAvg {
    pub category: String,
    pub avg: f64,
}

/// Rating distribution row
#[derive(Debug, FromRow, Serialize)]
pub struct RatingDistribution {
    pub rating: i32,
    pub count: i64,
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
    pub event_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
    /// Optional when using Clerk auth (user derived from JWT). Required in dev mode.
    pub user_id: Option<Uuid>,

    #[validate(length(max = 200, message = "Review title must be under 200 characters"))]
    pub title: Option<String>,

    #[validate(length(min = 350, max = 5000, message = "Review body must be 350-5000 characters"))]
    pub body: String,

    pub would_return: Option<bool>,

    pub category_ratings: HashMap<String, i16>,

    pub tag_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateReviewVote {
    /// Optional when using Clerk auth (user derived from JWT). Required in dev mode.
    pub user_id: Option<Uuid>,
    pub helpful: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateReviewComment {
    /// Optional when using Clerk auth (user derived from JWT). Required in dev mode.
    pub user_id: Option<Uuid>,
    pub parent_comment_id: Option<Uuid>,

    #[validate(length(min = 1, max = 2000, message = "Comment must be 1-2000 characters"))]
    pub body: String,
}
