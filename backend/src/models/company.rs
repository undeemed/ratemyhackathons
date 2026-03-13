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
}

#[derive(Debug, Serialize, FromRow)]
pub struct CompanyEventRef {
    pub id: Uuid,
    pub name: String,
    pub role: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub avg_rating: Option<f64>,
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
