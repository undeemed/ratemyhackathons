use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize)]
pub struct EventSponsor {
    pub id: Uuid,
    pub name: String,
    pub logo_url: Option<String>,
}
