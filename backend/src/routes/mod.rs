pub mod events;
pub mod companies;
pub mod users;
pub mod reviews;
pub mod search;
pub mod tags;
pub mod compare;

use serde::Serialize;

/// Paginated response wrapper
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

/// Pagination query parameters
#[derive(Debug, serde::Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

impl PaginationParams {
    pub fn offset(&self) -> i64 {
        (self.page() - 1) * self.limit()
    }

    pub fn limit(&self) -> i64 {
        self.per_page.unwrap_or(20).min(100)
    }

    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }
}
