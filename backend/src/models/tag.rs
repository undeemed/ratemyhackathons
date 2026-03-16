use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, FromRow, Serialize)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, FromRow, Serialize)]
pub struct TagCount {
    pub name: String,
    pub count: i64,
}

#[derive(Debug, FromRow, Serialize)]
pub struct TagVote {
    pub id: Uuid,
    pub tag_id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTag {
    #[validate(length(min = 1, max = 50, message = "Tag name must be 1-50 characters"))]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct VoteTag {
    pub user_id: Option<Uuid>,
}
