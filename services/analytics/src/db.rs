use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

// ── Crawl Stats ──

#[derive(Serialize)]
pub struct CrawlStats {
    pub total: i64,
    pub last_24h: i64,
    pub last_7d: i64,
    pub last_30d: i64,
    pub by_source: Vec<SourceCount>,
}

#[derive(Serialize)]
pub struct SourceCount {
    pub source_type: String,
    pub count: i64,
}

pub async fn crawl_stats(pool: &PgPool) -> Result<CrawlStats, sqlx::Error> {
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM crawl_sources")
        .fetch_one(pool)
        .await?;

    let last_24h: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM crawl_sources WHERE crawled_at > NOW() - INTERVAL '24 hours'"
    ).fetch_one(pool).await?;

    let last_7d: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM crawl_sources WHERE crawled_at > NOW() - INTERVAL '7 days'"
    ).fetch_one(pool).await?;

    let last_30d: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM crawl_sources WHERE crawled_at > NOW() - INTERVAL '30 days'"
    ).fetch_one(pool).await?;

    let by_source = sqlx::query_as::<_, (String, i64)>(
        "SELECT source_type, COUNT(*) FROM crawl_sources GROUP BY source_type ORDER BY source_type"
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|(source_type, count)| SourceCount { source_type, count })
    .collect();

    Ok(CrawlStats {
        total: total.0,
        last_24h: last_24h.0,
        last_7d: last_7d.0,
        last_30d: last_30d.0,
        by_source,
    })
}

// ── Crawl History (time-series) ──

#[derive(Serialize, sqlx::FromRow)]
pub struct CrawlDay {
    pub day: Option<NaiveDate>,
    pub count: Option<i64>,
}

pub async fn crawl_history(pool: &PgPool, days: i32) -> Result<Vec<CrawlDay>, sqlx::Error> {
    sqlx::query_as::<_, CrawlDay>(
        "SELECT date_trunc('day', crawled_at)::date as day, COUNT(*) as count \
         FROM crawl_sources \
         WHERE crawled_at > NOW() - ($1 || ' days')::interval \
         GROUP BY day ORDER BY day"
    )
    .bind(days.to_string())
    .fetch_all(pool)
    .await
}

// ── Source Registry ──

#[derive(Serialize, sqlx::FromRow)]
pub struct SourceInfo {
    pub id: Uuid,
    pub name: String,
    pub source_type: String,
    pub base_url: String,
    pub enabled: bool,
    pub poll_interval_hours: i32,
    pub last_polled_at: Option<DateTime<Utc>>,
    pub event_count: Option<i64>,
}

pub async fn crawl_sources(pool: &PgPool) -> Result<Vec<SourceInfo>, sqlx::Error> {
    sqlx::query_as::<_, SourceInfo>(
        "SELECT s.id, s.name, s.source_type, s.base_url, s.enabled, \
                s.poll_interval_hours, s.last_polled_at, \
                (SELECT COUNT(*) FROM crawl_sources c \
                 WHERE c.source_type = s.source_type) as event_count \
         FROM scrape_sources s ORDER BY s.name"
    )
    .fetch_all(pool)
    .await
}

// ── Event Analytics ──

#[derive(Serialize, sqlx::FromRow)]
pub struct TrendingEvent {
    pub id: Uuid,
    pub name: String,
    pub review_count: Option<i64>,
    pub avg_rating: Option<f64>,
}

pub async fn trending_events(
    pool: &PgPool,
    days: i32,
    limit: i32,
) -> Result<Vec<TrendingEvent>, sqlx::Error> {
    sqlx::query_as::<_, TrendingEvent>(
        "SELECT e.id, e.name, COUNT(r.id) as review_count, \
                AVG(r.rating)::float8 as avg_rating \
         FROM events e JOIN reviews r ON r.event_id = e.id \
         WHERE r.created_at > NOW() - ($1 || ' days')::interval \
         GROUP BY e.id ORDER BY review_count DESC LIMIT $2"
    )
    .bind(days.to_string())
    .bind(limit)
    .fetch_all(pool)
    .await
}

#[derive(Serialize, sqlx::FromRow)]
pub struct EventWeek {
    pub week: Option<NaiveDate>,
    pub count: Option<i64>,
}

pub async fn events_timeline(pool: &PgPool) -> Result<Vec<EventWeek>, sqlx::Error> {
    sqlx::query_as::<_, EventWeek>(
        "SELECT date_trunc('week', created_at)::date as week, COUNT(*) as count \
         FROM events GROUP BY week ORDER BY week"
    )
    .fetch_all(pool)
    .await
}

// ── Review Analytics ──

#[derive(Serialize, sqlx::FromRow)]
pub struct RatingBucket {
    pub rating: Option<i32>,
    pub count: Option<i64>,
}

pub async fn rating_distribution(pool: &PgPool) -> Result<Vec<RatingBucket>, sqlx::Error> {
    sqlx::query_as::<_, RatingBucket>(
        "SELECT rating, COUNT(*) as count FROM reviews GROUP BY rating ORDER BY rating"
    )
    .fetch_all(pool)
    .await
}

#[derive(Serialize, sqlx::FromRow)]
pub struct RecentReview {
    pub id: Uuid,
    pub entity_name: Option<String>,
    pub username: Option<String>,
    pub rating: i32,
    pub title: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub async fn recent_reviews(
    pool: &PgPool,
    limit: i32,
) -> Result<Vec<RecentReview>, sqlx::Error> {
    sqlx::query_as::<_, RecentReview>(
        "SELECT r.id, \
                COALESCE(e.name, c.name) as entity_name, \
                u.username, \
                r.rating, r.title, r.created_at \
         FROM reviews r \
         LEFT JOIN events e ON e.id = r.event_id \
         LEFT JOIN companies c ON c.id = r.company_id \
         JOIN users u ON u.id = r.user_id \
         ORDER BY r.created_at DESC LIMIT $1"
    )
    .bind(limit)
    .fetch_all(pool)
    .await
}
