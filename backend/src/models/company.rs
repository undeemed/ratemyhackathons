use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

// ── Database row ──

#[derive(Debug, FromRow, Serialize)]
pub struct Company {
    pub id: Uuid,
    pub name: String,
    pub logo_url: Option<String>,
    pub website: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ── API responses ──

#[derive(Debug, Serialize, FromRow)]
pub struct CompanySummary {
    pub id: Uuid,
    pub name: String,
    pub logo_url: Option<String>,
    pub website: Option<String>,
    pub description: Option<String>,
    pub event_count: i64,
    pub avg_rating: Option<f64>,
    pub review_count: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CompanySummaryResponse {
    pub id: Uuid,
    pub name: String,
    pub logo_url: Option<String>,
    pub website: Option<String>,
    pub description: Option<String>,
    pub event_count: i64,
    pub avg_rating: Option<f64>,
    pub review_count: i64,
    pub category_ratings: Vec<crate::models::review::CategoryAvg>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CompanyDetail {
    pub id: Uuid,
    pub name: String,
    pub logo_url: Option<String>,
    pub website: Option<String>,
    pub description: Option<String>,
    pub events: Vec<CompanyEventRef>,
    pub avg_rating: Option<f64>,
    pub review_count: i64,
    pub would_return_pct: Option<f64>,
    pub category_ratings: Vec<crate::models::review::CategoryAvg>,
    pub top_tags: Vec<crate::models::tag::TagCount>,
    pub rating_distribution: Vec<crate::models::review::RatingDistribution>,
    pub reviews: Vec<CompanyReviewRef>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct CompanyEventRef {
    pub id: Uuid,
    pub name: String,
    pub role: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub avg_rating: Option<f64>,
}

#[derive(Debug, FromRow)]
pub struct CompanyReviewRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub rating: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub would_return: Option<bool>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CompanyReviewRef {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub rating: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub would_return: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub category_ratings: Vec<crate::models::review::ReviewRatingRow>,
}

// ── API requests ──

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCompany {
    #[validate(length(min = 1, max = 200, message = "Company name must be 1-200 characters"))]
    pub name: String,

    #[validate(url(message = "Invalid logo URL"))]
    pub logo_url: Option<String>,

    #[validate(url(message = "Invalid website URL"))]
    pub website: Option<String>,

    #[validate(length(max = 2000, message = "Description must be under 2000 characters"))]
    pub description: Option<String>,
}
