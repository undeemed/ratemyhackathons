use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Database row ──

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub age: Option<i32>,
    pub avatar_url: Option<String>,
    pub github: Option<String>,
    pub twitter: Option<String>,
    pub linkedin: Option<String>,
    pub website: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ── API responses ──

#[derive(Debug, Serialize, FromRow)]
pub struct UserSummary {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub review_count: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UserDetail {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub age: Option<i32>,
    pub avatar_url: Option<String>,
    pub socials: UserSocials,
    pub reviews: Vec<UserReviewRef>,
}

#[derive(Debug, Serialize)]
pub struct UserSocials {
    pub github: Option<String>,
    pub twitter: Option<String>,
    pub linkedin: Option<String>,
    pub website: Option<String>,
}

/// Row for review joins (proper FromRow instead of tuple)
#[derive(Debug, FromRow)]
pub struct UserReviewRow {
    pub id: Uuid,
    pub event_id: Uuid,
    pub event_name: String,
    pub rating: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UserReviewRef {
    pub id: Uuid,
    pub event_id: Uuid,
    pub event_name: String,
    pub rating: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ── API requests ──

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub age: Option<i32>,
    pub avatar_url: Option<String>,
    pub github: Option<String>,
    pub twitter: Option<String>,
    pub linkedin: Option<String>,
    pub website: Option<String>,
}
