use chrono::{NaiveDate, DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

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

#[derive(Debug, Deserialize)]
pub struct CreateEvent {
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub url: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub image_url: Option<String>,
    pub company_ids: Option<Vec<Uuid>>,
}
