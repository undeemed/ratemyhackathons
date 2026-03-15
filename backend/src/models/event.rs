use chrono::{NaiveDate, DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

// ── Database rows ──

#[derive(Debug, FromRow, Serialize)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub url: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub image_url: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Row for list queries with aggregated review stats
#[derive(Debug, FromRow)]
pub struct EventRow {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub url: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub image_url: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub avg_rating: Option<f64>,
    pub review_count: i64,
}

/// Row for batch-fetched event-company relationships
#[derive(Debug, FromRow)]
pub struct EventCompanyRow {
    pub event_id: Uuid,
    pub id: Uuid,
    pub name: String,
    pub role: Option<String>,
}

/// Row for review joins (replaces ugly tuple destructuring)
#[derive(Debug, FromRow)]
pub struct EventReviewRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub rating: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ── API responses ──

#[derive(Debug, Serialize)]
pub struct EventSummary {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub url: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub image_url: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub companies: Vec<EventCompanyRef>,
    pub avg_rating: Option<f64>,
    pub review_count: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct EventDetail {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub url: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub image_url: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub companies: Vec<EventCompanyRef>,
    pub reviews: Vec<EventReviewRef>,
    pub avg_rating: Option<f64>,
    pub review_count: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct EventCompanyRef {
    pub id: Uuid,
    pub name: String,
    pub role: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EventReviewRef {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub rating: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ── API requests ──

#[derive(Debug, Deserialize, Validate)]
pub struct CreateEvent {
    #[validate(length(min = 1, max = 200, message = "Event name must be 1-200 characters"))]
    pub name: String,

    #[validate(length(max = 5000, message = "Description must be under 5000 characters"))]
    pub description: Option<String>,

    #[validate(length(max = 200, message = "Location must be under 200 characters"))]
    pub location: Option<String>,

    #[validate(url(message = "Invalid event URL"))]
    pub url: Option<String>,

    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,

    #[validate(url(message = "Invalid image URL"))]
    pub image_url: Option<String>,

    pub company_ids: Option<Vec<Uuid>>,

    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

/// Lightweight marker for globe visualization
#[derive(Debug, Serialize, FromRow)]
pub struct GlobeMarker {
    pub id: Uuid,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub start_date: Option<NaiveDate>,
}
