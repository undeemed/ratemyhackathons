use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

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

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 1, max = 30, message = "Username must be 1-30 characters"))]
    pub username: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(max = 100, message = "Display name must be under 100 characters"))]
    pub display_name: Option<String>,

    #[validate(length(max = 500, message = "Bio must be under 500 characters"))]
    pub bio: Option<String>,

    #[validate(range(min = 13, max = 150, message = "Age must be between 13 and 150"))]
    pub age: Option<i32>,

    #[validate(url(message = "Invalid avatar URL"))]
    pub avatar_url: Option<String>,

    #[validate(length(max = 39, message = "GitHub username must be under 39 characters"))]
    pub github: Option<String>,

    #[validate(length(max = 50, message = "Twitter handle must be under 50 characters"))]
    pub twitter: Option<String>,

    #[validate(length(max = 100, message = "LinkedIn slug must be under 100 characters"))]
    pub linkedin: Option<String>,

    #[validate(url(message = "Invalid website URL"))]
    pub website: Option<String>,
}
