# Phase 1: Schema + API Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add multi-dimensional ratings (10 categories), tags, and enhanced search/detail/review/compare endpoints to the Rust/Actix-Web backend.

**Architecture:** New migration adds `review_ratings`, `tags`, `review_tags` tables and alters `reviews` (nullable `event_id`, new `company_id`, `would_return`) and `users` (`clerk_id`). Existing route modules get enhanced queries; new `tags` and `compare` route modules are added. All SQL uses correlated subqueries to avoid N+1.

**Tech Stack:** Rust 2024 edition, Actix-Web 4, sqlx 0.8 (Postgres), serde, uuid v7, validator, ammonia

**Spec:** `docs/superpowers/specs/2026-03-14-rmp-style-flows-design.md`

---

## File Structure

### New Files
- `backend/migrations/20260314_rmp_ratings.sql` — Schema migration for review_ratings, tags, review_tags, reviews alterations, users clerk_id
- `backend/src/models/tag.rs` — Tag and CreateTag DTOs
- `backend/src/routes/tags.rs` — GET /api/tags, GET /api/tags/top, POST /api/tags
- `backend/src/routes/compare.rs` — GET /api/compare

### Modified Files
- `backend/src/models/mod.rs` — Add `pub mod tag;`
- `backend/src/models/review.rs` — Add `company_id`, `would_return`, `ReviewRatingRow`, `CategoryRating`, update `CreateReview`
- `backend/src/models/company.rs` — Add `CompanyReviewRef`, `CompanyReviewRow`, enhance `CompanyDetail`
- `backend/src/models/event.rs` — Enhance `EventDetail`, `EventReviewRef`
- `backend/src/models/user.rs` — Update `UserReviewRow` for nullable event_id, add company review support
- `backend/src/routes/mod.rs` — Add `pub mod tags;` and `pub mod compare;`
- `backend/src/routes/search.rs` — Add `avg_rating`, `review_count`, `would_return_pct` to SearchResult
- `backend/src/routes/events.rs` — Add category_ratings, top_tags, rating_distribution, would_return_pct to event detail
- `backend/src/routes/companies.rs` — Add category_ratings, top_tags, rating_distribution, would_return_pct, reviews to company detail
- `backend/src/routes/reviews.rs` — Include category_ratings in get_review response
- `backend/src/routes/users.rs` — Rewrite `create_review` for multi-dimensional ratings with category scores + tags
- `backend/src/main.rs` — Register new tag and compare routes

---

## Chunk 1: Schema Migration + Tag Model/Routes

### Task 1: Write the schema migration

**Files:**
- Create: `backend/migrations/20260314_rmp_ratings.sql`

- [ ] **Step 1: Write the migration SQL**

```sql
-- RateMyHackathons Phase 1: Multi-dimensional ratings, tags, company reviews
-- Depends on: 20260314_event_geocoding.sql

-- ═══════════════════════════════════════════════
-- 1. Review ratings (per-category scores)
-- ═══════════════════════════════════════════════

CREATE TABLE review_ratings (
    review_id   UUID NOT NULL REFERENCES reviews(id) ON DELETE CASCADE,
    category    TEXT NOT NULL,
    score       SMALLINT NOT NULL CHECK (score BETWEEN 1 AND 5),
    PRIMARY KEY (review_id, category)
);

-- ═══════════════════════════════════════════════
-- 2. Tags (crowd-sourced labels)
-- ═══════════════════════════════════════════════

CREATE TABLE tags (
    id   UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE review_tags (
    review_id UUID NOT NULL REFERENCES reviews(id) ON DELETE CASCADE,
    tag_id    UUID NOT NULL REFERENCES tags(id),
    PRIMARY KEY (review_id, tag_id)
);

-- ═══════════════════════════════════════════════
-- 3. ALTER reviews: add company_id, would_return, make event_id nullable
-- ═══════════════════════════════════════════════

-- Drop existing unique constraint (event_id, user_id) so we can make event_id nullable
ALTER TABLE reviews DROP CONSTRAINT IF EXISTS reviews_event_id_user_id_key;

ALTER TABLE reviews
    ADD COLUMN company_id UUID REFERENCES companies(id),
    ADD COLUMN would_return BOOLEAN,
    ALTER COLUMN event_id DROP NOT NULL;

-- XOR: exactly one of event_id or company_id must be set
ALTER TABLE reviews ADD CONSTRAINT reviews_target_xor CHECK (
    (event_id IS NOT NULL AND company_id IS NULL) OR
    (event_id IS NULL AND company_id IS NOT NULL)
);

-- Partial unique indexes: one review per user per event/company
CREATE UNIQUE INDEX idx_reviews_event_user_unique
    ON reviews (event_id, user_id) WHERE event_id IS NOT NULL;
CREATE UNIQUE INDEX idx_reviews_company_user_unique
    ON reviews (company_id, user_id) WHERE company_id IS NOT NULL;

-- ═══════════════════════════════════════════════
-- 4. ALTER users: add clerk_id for Clerk auth (Phase 2 prep)
-- ═══════════════════════════════════════════════

ALTER TABLE users ADD COLUMN IF NOT EXISTS clerk_id TEXT UNIQUE;

-- ═══════════════════════════════════════════════
-- 5. Indexes
-- ═══════════════════════════════════════════════

CREATE INDEX idx_review_ratings_review ON review_ratings (review_id);
CREATE INDEX idx_review_ratings_category ON review_ratings (category);
CREATE INDEX idx_review_tags_review ON review_tags (review_id);
CREATE INDEX idx_review_tags_tag ON review_tags (tag_id);
CREATE INDEX idx_reviews_company ON reviews (company_id);
CREATE INDEX idx_users_clerk ON users (clerk_id);
```

- [ ] **Step 2: Apply the migration**

Run:
```bash
psql -d ratemyhackathons -f backend/migrations/20260314_rmp_ratings.sql
```

Expected: All statements execute without error. Verify with:
```bash
psql -d ratemyhackathons -c "\dt review_ratings" -c "\dt tags" -c "\dt review_tags" -c "\d reviews"
```

- [ ] **Step 3: Commit**

```bash
git add backend/migrations/20260314_rmp_ratings.sql
git commit -m "feat(schema): add review_ratings, tags, company reviews migration"
```

---

### Task 2: Create tag model

**Files:**
- Create: `backend/src/models/tag.rs`
- Modify: `backend/src/models/mod.rs`

- [ ] **Step 1: Create `backend/src/models/tag.rs`**

```rust
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

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTag {
    #[validate(length(min = 1, max = 50, message = "Tag name must be 1-50 characters"))]
    pub name: String,
}
```

- [ ] **Step 2: Register the module in `backend/src/models/mod.rs`**

Add `pub mod tag;` after the existing module declarations:

```rust
pub mod event;
pub mod company;
pub mod user;
pub mod review;
pub mod tag;
```

- [ ] **Step 3: Verify compilation**

Run: `cd backend && cargo check`
Expected: Compiles without errors.

- [ ] **Step 4: Commit**

```bash
git add backend/src/models/tag.rs backend/src/models/mod.rs
git commit -m "feat(models): add Tag, TagCount, CreateTag DTOs"
```

---

### Task 3: Create tag routes

**Files:**
- Create: `backend/src/routes/tags.rs`
- Modify: `backend/src/routes/mod.rs`
- Modify: `backend/src/main.rs`

- [ ] **Step 1: Create `backend/src/routes/tags.rs`**

```rust
use actix_web::{get, post, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::errors::ApiError;
use crate::models::tag::*;

/// GET /api/tags — List all tags alphabetically
#[get("/tags")]
pub async fn list_tags(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiError> {
    let tags = sqlx::query_as::<_, Tag>(
        "SELECT id, name FROM tags ORDER BY name ASC",
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(tags))
}

#[derive(Debug, serde::Deserialize)]
pub struct TopTagsParams {
    pub entity_type: String,  // "event" or "company"
    pub entity_id: Uuid,
}

/// GET /api/tags/top?entity_type=event&entity_id=uuid — Top 5 tags for an entity
#[get("/tags/top")]
pub async fn top_tags(
    pool: web::Data<PgPool>,
    query: web::Query<TopTagsParams>,
) -> Result<HttpResponse, ApiError> {
    let tags = match query.entity_type.as_str() {
        "event" => {
            sqlx::query_as::<_, TagCount>(
                r#"
                SELECT t.name, COUNT(*) as count
                FROM review_tags rt
                JOIN tags t ON t.id = rt.tag_id
                JOIN reviews r ON r.id = rt.review_id
                WHERE r.event_id = $1
                GROUP BY t.name
                ORDER BY count DESC
                LIMIT 5
                "#,
            )
            .bind(query.entity_id)
            .fetch_all(pool.get_ref())
            .await?
        }
        "company" => {
            sqlx::query_as::<_, TagCount>(
                r#"
                SELECT t.name, COUNT(*) as count
                FROM review_tags rt
                JOIN tags t ON t.id = rt.tag_id
                JOIN reviews r ON r.id = rt.review_id
                WHERE r.company_id = $1
                GROUP BY t.name
                ORDER BY count DESC
                LIMIT 5
                "#,
            )
            .bind(query.entity_id)
            .fetch_all(pool.get_ref())
            .await?
        }
        _ => {
            return Err(ApiError::BadRequest(
                "entity_type must be 'event' or 'company'".to_string(),
            ));
        }
    };

    Ok(HttpResponse::Ok().json(tags))
}

/// POST /api/tags — Create a new tag (returns existing if name matches)
#[post("/tags")]
pub async fn create_tag(
    pool: web::Data<PgPool>,
    body: web::Json<CreateTag>,
) -> Result<HttpResponse, ApiError> {
    body.validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let name = ammonia::clean(&body.name).to_lowercase();

    // Return existing tag if name matches (case-insensitive)
    if let Some(existing) = sqlx::query_as::<_, Tag>(
        "SELECT id, name FROM tags WHERE LOWER(name) = $1",
    )
    .bind(&name)
    .fetch_optional(pool.get_ref())
    .await?
    {
        return Ok(HttpResponse::Ok().json(existing));
    }

    let id = Uuid::now_v7();
    let tag = sqlx::query_as::<_, Tag>(
        "INSERT INTO tags (id, name) VALUES ($1, $2) RETURNING id, name",
    )
    .bind(id)
    .bind(&name)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(tag))
}
```

- [ ] **Step 2: Register module in `backend/src/routes/mod.rs`**

Add `pub mod tags;` and `pub mod compare;` after existing module declarations:

```rust
pub mod events;
pub mod companies;
pub mod users;
pub mod reviews;
pub mod search;
pub mod tags;
pub mod compare;
```

- [ ] **Step 3: Register routes in `backend/src/main.rs`**

Add inside the `/api` scope, after the existing `.service()` calls:

```rust
.service(routes::tags::list_tags)
.service(routes::tags::top_tags)
.service(routes::tags::create_tag)
.service(routes::compare::compare)
```

Note: `routes::compare::compare` will be created in Task 7. For now, create a placeholder `backend/src/routes/compare.rs`:

```rust
use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;

use crate::errors::ApiError;

#[derive(Debug, serde::Deserialize)]
pub struct CompareParams {
    #[serde(rename = "type")]
    pub entity_type: String,
    pub ids: String,
}

/// GET /api/compare?type=company&ids=uuid1,uuid2
#[get("/compare")]
pub async fn compare(
    _pool: web::Data<PgPool>,
    _query: web::Query<CompareParams>,
) -> Result<HttpResponse, ApiError> {
    // Placeholder — implemented in Task 7
    Ok(HttpResponse::Ok().json(serde_json::json!({"status": "not_implemented"})))
}
```

- [ ] **Step 4: Verify compilation**

Run: `cd backend && cargo check`
Expected: Compiles without errors.

- [ ] **Step 5: Commit**

```bash
git add backend/src/routes/tags.rs backend/src/routes/compare.rs backend/src/routes/mod.rs backend/src/main.rs
git commit -m "feat(api): add tag CRUD endpoints and compare placeholder"
```

---

## Chunk 2: Enhanced Review Model + Create Review

### Task 4: Update review model for multi-dimensional ratings

**Files:**
- Modify: `backend/src/models/review.rs`

- [ ] **Step 1: Add new types and update existing structs**

Add these new structs after the existing ones in `review.rs`:

```rust
use std::collections::HashMap;
```

Add to the imports at top.

Add `company_id` and `would_return` to the `Review` struct:

```rust
#[derive(Debug, FromRow, Serialize)]
pub struct Review {
    pub id: Uuid,
    pub event_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
    pub user_id: Uuid,
    pub rating: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub would_return: Option<bool>,
    pub created_at: DateTime<Utc>,
}
```

Add new DTOs after the existing structs:

```rust
/// Per-category rating row from review_ratings table
#[derive(Debug, FromRow, Serialize)]
pub struct ReviewRatingRow {
    pub category: String,
    pub score: i16,
}

/// Category average for aggregated display
#[derive(Debug, FromRow, Serialize, Clone)]
pub struct CategoryAvg {
    pub category: String,
    pub avg: f64,
}

/// Rating distribution row
#[derive(Debug, FromRow, Serialize)]
pub struct RatingDistribution {
    pub rating: i32,
    pub count: i64,
}
```

Update `CreateReview` to support multi-dimensional ratings:

```rust
#[derive(Debug, Deserialize, Validate)]
pub struct CreateReview {
    pub event_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
    pub user_id: Uuid,

    #[validate(length(max = 200, message = "Review title must be under 200 characters"))]
    pub title: Option<String>,

    #[validate(length(min = 350, max = 5000, message = "Review body must be 350-5000 characters"))]
    pub body: String,

    pub would_return: Option<bool>,

    pub category_ratings: HashMap<String, i16>,

    pub tag_ids: Option<Vec<Uuid>>,
}
```

Note: The `rating: i32` field is removed from `CreateReview` — it's now computed from category averages. The `body` field is now required (min 350 chars per spec).

- [ ] **Step 2: Verify compilation**

Run: `cd backend && cargo check`
Expected: Compilation errors from routes that reference old `CreateReview` fields — these are fixed in later tasks.

- [ ] **Step 3: Commit**

```bash
git add backend/src/models/review.rs
git commit -m "feat(models): update Review for multi-dimensional ratings"
```

---

### Task 5: Update user model for nullable event reviews

**Files:**
- Modify: `backend/src/models/user.rs`

- [ ] **Step 1: Read current user model**

Read `backend/src/models/user.rs` to see current structs.

- [ ] **Step 2: Update `UserReviewRow` and `UserReviewRef`**

Change `event_id: Uuid` to `event_id: Option<Uuid>` and add `company_id`:

```rust
#[derive(Debug, FromRow)]
pub struct UserReviewRow {
    pub id: Uuid,
    pub event_id: Option<Uuid>,
    pub event_name: Option<String>,
    pub company_id: Option<Uuid>,
    pub company_name: Option<String>,
    pub rating: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub created_at: DateTime<Utc>,
}
```

Update the corresponding `UserReviewRef`:

```rust
#[derive(Debug, Serialize)]
pub struct UserReviewRef {
    pub id: Uuid,
    pub event_id: Option<Uuid>,
    pub event_name: Option<String>,
    pub company_id: Option<Uuid>,
    pub company_name: Option<String>,
    pub rating: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub created_at: DateTime<Utc>,
}
```

- [ ] **Step 3: Verify compilation**

Run: `cd backend && cargo check`
Expected: Errors from `routes/users.rs` query that JOINs events — fixed in Task 6.

- [ ] **Step 4: Commit**

```bash
git add backend/src/models/user.rs
git commit -m "feat(models): update UserReviewRow for company reviews"
```

---

### Task 6: Rewrite create_review and get_user for multi-dimensional ratings

**Files:**
- Modify: `backend/src/routes/users.rs`

- [ ] **Step 1: Update the `create_review` handler**

Replace the entire `create_review` function in `routes/users.rs`:

```rust
const VALID_CATEGORIES: &[&str] = &[
    "organization", "prizes", "mentorship", "judging", "venue",
    "food", "swag", "networking", "communication", "vibes",
];

#[post("/reviews")]
pub async fn create_review(
    pool: web::Data<PgPool>,
    body: web::Json<CreateReview>,
) -> Result<HttpResponse, ApiError> {
    body.validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    // Validate XOR: exactly one target
    match (&body.event_id, &body.company_id) {
        (Some(_), None) | (None, Some(_)) => {}
        _ => {
            return Err(ApiError::BadRequest(
                "Exactly one of event_id or company_id must be provided".to_string(),
            ));
        }
    }

    // Validate all 10 categories present and scores in range
    if body.category_ratings.len() != 10 {
        return Err(ApiError::BadRequest(
            "All 10 category ratings are required".to_string(),
        ));
    }
    for cat in VALID_CATEGORIES {
        match body.category_ratings.get(*cat) {
            Some(&score) if (1..=5).contains(&score) => {}
            Some(_) => {
                return Err(ApiError::BadRequest(
                    format!("Category '{}' score must be between 1 and 5", cat),
                ));
            }
            None => {
                return Err(ApiError::BadRequest(
                    format!("Missing required category: {}", cat),
                ));
            }
        }
    }

    // Compute overall rating as average of category scores
    let total: i16 = body.category_ratings.values().sum();
    let rating = ((total as f64 / 10.0).round()) as i32;

    // Verify target entity and user exist
    if let Some(event_id) = body.event_id {
        let exists: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM events WHERE id = $1")
            .bind(event_id)
            .fetch_optional(pool.get_ref())
            .await?;
        if exists.is_none() {
            return Err(ApiError::NotFound(format!("Event {} not found", event_id)));
        }
    }
    if let Some(company_id) = body.company_id {
        let exists: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM companies WHERE id = $1")
            .bind(company_id)
            .fetch_optional(pool.get_ref())
            .await?;
        if exists.is_none() {
            return Err(ApiError::NotFound(format!("Company {} not found", company_id)));
        }
    }
    let user_exists: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM users WHERE id = $1")
        .bind(body.user_id)
        .fetch_optional(pool.get_ref())
        .await?;
    if user_exists.is_none() {
        return Err(ApiError::NotFound(format!("User {} not found", body.user_id)));
    }

    let id = Uuid::now_v7();

    // Sanitize free-text fields
    let title = body.title.as_deref().map(|s| ammonia::clean(s));
    let review_body = ammonia::clean(&body.body);

    // Transaction: insert review + 10 category ratings + tag links
    let mut tx = pool.begin().await?;

    sqlx::query(
        r#"
        INSERT INTO reviews (id, event_id, company_id, user_id, rating, title, body, would_return)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(id)
    .bind(body.event_id)
    .bind(body.company_id)
    .bind(body.user_id)
    .bind(rating)
    .bind(&title)
    .bind(&review_body)
    .bind(body.would_return)
    .execute(&mut *tx)
    .await?;

    // Batch-insert category ratings
    for (category, score) in &body.category_ratings {
        sqlx::query(
            "INSERT INTO review_ratings (review_id, category, score) VALUES ($1, $2, $3)",
        )
        .bind(id)
        .bind(category)
        .bind(score)
        .execute(&mut *tx)
        .await?;
    }

    // Link tags if provided
    if let Some(ref tag_ids) = body.tag_ids {
        for tag_id in tag_ids {
            sqlx::query(
                "INSERT INTO review_tags (review_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            )
            .bind(id)
            .bind(tag_id)
            .execute(&mut *tx)
            .await?;
        }
    }

    tx.commit().await?;

    // Fetch the created review
    let review = sqlx::query_as::<_, Review>(
        "SELECT id, event_id, company_id, user_id, rating, title, body, would_return, created_at FROM reviews WHERE id = $1",
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(review))
}
```

- [ ] **Step 2: Update the `get_user` query for reviews**

Replace the reviews query in `get_user` to LEFT JOIN both events and companies:

```rust
    let reviews = sqlx::query_as::<_, UserReviewRow>(
        r#"
        SELECT r.id, r.event_id, e.name as event_name,
               r.company_id, c.name as company_name,
               r.rating, r.title, r.body, r.created_at
        FROM reviews r
        LEFT JOIN events e ON e.id = r.event_id
        LEFT JOIN companies c ON c.id = r.company_id
        WHERE r.user_id = $1
        ORDER BY r.created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool.get_ref())
    .await?;
```

And update the mapping in `get_user`:

```rust
        reviews: reviews.into_iter().map(|r| UserReviewRef {
            id: r.id,
            event_id: r.event_id,
            event_name: r.event_name,
            company_id: r.company_id,
            company_name: r.company_name,
            rating: r.rating,
            title: r.title,
            body: r.body,
            created_at: r.created_at,
        }).collect(),
```

- [ ] **Step 3: Verify compilation**

Run: `cd backend && cargo check`
Expected: Compiles successfully.

- [ ] **Step 4: Commit**

```bash
git add backend/src/routes/users.rs
git commit -m "feat(api): rewrite create_review for multi-dimensional ratings"
```

---

## Chunk 3: Enhanced Detail Endpoints

### Task 7: Enhance company detail endpoint

**Files:**
- Modify: `backend/src/models/company.rs`
- Modify: `backend/src/routes/companies.rs`

- [ ] **Step 1: Add new types to `backend/src/models/company.rs`**

Add these after existing structs:

```rust
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
```

Enhance `CompanyDetail`:

```rust
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
```

- [ ] **Step 2: Enhance the `get_company` handler in `backend/src/routes/companies.rs`**

Replace the `get_company` function:

```rust
use crate::models::review::{CategoryAvg, RatingDistribution, ReviewRatingRow};
use crate::models::tag::TagCount;
```

Add these imports at the top.

```rust
#[get("/companies/{id}")]
pub async fn get_company(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let company_id = id.into_inner();

    let company = sqlx::query_as::<_, Company>(
        "SELECT id, name, logo_url, website, description, created_at FROM companies WHERE id = $1",
    )
    .bind(company_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("Company {} not found", company_id)))?;

    let events = sqlx::query_as::<_, CompanyEventRef>(
        r#"
        SELECT e.id, e.name, ec.role, e.start_date,
               (SELECT AVG(rating)::float8 FROM reviews WHERE event_id = e.id) as avg_rating
        FROM events e
        JOIN event_companies ec ON ec.event_id = e.id
        WHERE ec.company_id = $1
        ORDER BY e.start_date DESC NULLS LAST
        "#,
    )
    .bind(company_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Aggregate stats
    let stats: (Option<f64>, i64) = sqlx::query_as(
        "SELECT AVG(rating)::float8, COUNT(*) FROM reviews WHERE company_id = $1",
    )
    .bind(company_id)
    .fetch_one(pool.get_ref())
    .await?;

    // Would return percentage
    let would_return: (Option<f64>,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FILTER (WHERE would_return = true) * 100.0 / NULLIF(COUNT(*), 0)
        FROM reviews WHERE company_id = $1
        "#,
    )
    .bind(company_id)
    .fetch_one(pool.get_ref())
    .await?;

    // Category averages
    let category_ratings = sqlx::query_as::<_, CategoryAvg>(
        r#"
        SELECT rr.category, AVG(rr.score)::float8 as avg
        FROM review_ratings rr
        JOIN reviews r ON r.id = rr.review_id
        WHERE r.company_id = $1
        GROUP BY rr.category
        "#,
    )
    .bind(company_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Top tags
    let top_tags = sqlx::query_as::<_, TagCount>(
        r#"
        SELECT t.name, COUNT(*) as count
        FROM review_tags rt
        JOIN tags t ON t.id = rt.tag_id
        JOIN reviews r ON r.id = rt.review_id
        WHERE r.company_id = $1
        GROUP BY t.name
        ORDER BY count DESC
        LIMIT 5
        "#,
    )
    .bind(company_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Rating distribution
    let rating_distribution = sqlx::query_as::<_, RatingDistribution>(
        r#"
        SELECT rating, COUNT(*) as count
        FROM reviews WHERE company_id = $1
        GROUP BY rating ORDER BY rating DESC
        "#,
    )
    .bind(company_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Reviews with per-review category ratings
    let review_rows = sqlx::query_as::<_, CompanyReviewRow>(
        r#"
        SELECT r.id, r.user_id, u.username, r.rating, r.title, r.body, r.would_return, r.created_at
        FROM reviews r
        JOIN users u ON u.id = r.user_id
        WHERE r.company_id = $1
        ORDER BY r.created_at DESC
        "#,
    )
    .bind(company_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Batch-fetch category ratings for all reviews
    let review_ids: Vec<Uuid> = review_rows.iter().map(|r| r.id).collect();
    let all_ratings = sqlx::query_as::<_, (Uuid, String, i16)>(
        "SELECT review_id, category, score FROM review_ratings WHERE review_id = ANY($1)",
    )
    .bind(&review_ids)
    .fetch_all(pool.get_ref())
    .await?;

    let reviews: Vec<CompanyReviewRef> = review_rows
        .into_iter()
        .map(|r| {
            let cats: Vec<ReviewRatingRow> = all_ratings
                .iter()
                .filter(|(rid, _, _)| *rid == r.id)
                .map(|(_, cat, score)| ReviewRatingRow {
                    category: cat.clone(),
                    score: *score,
                })
                .collect();
            CompanyReviewRef {
                id: r.id,
                user_id: r.user_id,
                username: r.username,
                rating: r.rating,
                title: r.title,
                body: r.body,
                would_return: r.would_return,
                created_at: r.created_at,
                category_ratings: cats,
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(CompanyDetail {
        id: company.id,
        name: company.name,
        logo_url: company.logo_url,
        website: company.website,
        description: company.description,
        events,
        avg_rating: stats.0,
        review_count: stats.1,
        would_return_pct: would_return.0,
        category_ratings,
        top_tags,
        rating_distribution,
        reviews,
    }))
}
```

- [ ] **Step 3: Verify compilation**

Run: `cd backend && cargo check`
Expected: Compiles without errors.

- [ ] **Step 4: Commit**

```bash
git add backend/src/models/company.rs backend/src/routes/companies.rs
git commit -m "feat(api): enhance company detail with category ratings, tags, distribution"
```

---

### Task 8: Enhance event detail endpoint

**Files:**
- Modify: `backend/src/models/event.rs`
- Modify: `backend/src/routes/events.rs`

- [ ] **Step 1: Add new fields to `EventReviewRef` and `EventDetail` in `backend/src/models/event.rs`**

Update `EventReviewRow`:

```rust
#[derive(Debug, FromRow)]
pub struct EventReviewRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub rating: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub would_return: Option<bool>,
    pub created_at: DateTime<Utc>,
}
```

Update `EventReviewRef`:

```rust
#[derive(Debug, Serialize)]
pub struct EventReviewRef {
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
```

Update `EventDetail`:

```rust
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
    pub would_return_pct: Option<f64>,
    pub category_ratings: Vec<crate::models::review::CategoryAvg>,
    pub top_tags: Vec<crate::models::tag::TagCount>,
    pub rating_distribution: Vec<crate::models::review::RatingDistribution>,
}
```

- [ ] **Step 2: Enhance the `get_event` handler in `backend/src/routes/events.rs`**

Add imports:

```rust
use crate::models::review::{CategoryAvg, RatingDistribution, ReviewRatingRow};
use crate::models::tag::TagCount;
```

Replace the `get_event` function with the enhanced version that adds category_ratings, top_tags, rating_distribution, would_return_pct, and per-review category scores. Follow the same pattern as the company detail (Task 7) but using `event_id` instead of `company_id`:

- Add `would_return` to the reviews SELECT
- Add category averages query
- Add top tags query
- Add rating distribution query
- Batch-fetch category ratings for reviews
- Assemble `EventReviewRef` with category_ratings

The `get_event` function body follows the exact same query pattern as `get_company` in Task 7 but filtering on `r.event_id = $1` instead of `r.company_id = $1`.

- [ ] **Step 3: Verify compilation**

Run: `cd backend && cargo check`
Expected: Compiles without errors.

- [ ] **Step 4: Commit**

```bash
git add backend/src/models/event.rs backend/src/routes/events.rs
git commit -m "feat(api): enhance event detail with category ratings, tags, distribution"
```

---

### Task 9: Update get_review to include category ratings

**Files:**
- Modify: `backend/src/routes/reviews.rs`

- [ ] **Step 1: Add category_ratings to get_review response**

After fetching the review and vote counts, add:

```rust
    // Category ratings for this review
    let category_ratings = sqlx::query_as::<_, crate::models::review::ReviewRatingRow>(
        "SELECT category, score FROM review_ratings WHERE review_id = $1",
    )
    .bind(review_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Tags for this review
    let tags = sqlx::query_as::<_, crate::models::tag::Tag>(
        r#"
        SELECT t.id, t.name
        FROM review_tags rt
        JOIN tags t ON t.id = rt.tag_id
        WHERE rt.review_id = $1
        "#,
    )
    .bind(review_id)
    .fetch_all(pool.get_ref())
    .await?;
```

Update the JSON response to include them:

```rust
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "id": review.id,
        "event_id": review.event_id,
        "company_id": review.company_id,
        "user_id": review.user_id,
        "rating": review.rating,
        "title": review.title,
        "body": review.body,
        "would_return": review.would_return,
        "created_at": review.created_at,
        "category_ratings": category_ratings,
        "tags": tags,
        "votes": {
            "helpful": helpful.0,
            "unhelpful": unhelpful.0
        },
        "comments": comments
    })))
```

- [ ] **Step 2: Verify compilation**

Run: `cd backend && cargo check`
Expected: Compiles without errors.

- [ ] **Step 3: Commit**

```bash
git add backend/src/routes/reviews.rs
git commit -m "feat(api): include category ratings and tags in review detail"
```

---

## Chunk 4: Enhanced Search + Compare

### Task 10: Enhance search endpoint with rating data

**Files:**
- Modify: `backend/src/routes/search.rs`

- [ ] **Step 1: Update `SearchResult` to include rating fields**

```rust
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SearchResult {
    pub id: Uuid,
    pub name: String,
    pub rank: f32,
    pub avg_rating: Option<f64>,
    pub review_count: i64,
    pub would_return_pct: Option<f64>,
}
```

- [ ] **Step 2: Update the event search query**

```rust
        events = sqlx::query_as::<_, SearchResult>(
            r#"
            SELECT e.id, e.name,
                   ts_rank(e.search_vector, plainto_tsquery('english', $1)) as rank,
                   rs.avg_rating,
                   COALESCE(rs.review_count, 0) as review_count,
                   rs.would_return_pct
            FROM events e
            LEFT JOIN LATERAL (
                SELECT AVG(rating)::float8 as avg_rating,
                       COUNT(*) as review_count,
                       COUNT(*) FILTER (WHERE would_return = true) * 100.0 / NULLIF(COUNT(*), 0) as would_return_pct
                FROM reviews WHERE event_id = e.id
            ) rs ON true
            WHERE e.search_vector @@ plainto_tsquery('english', $1)
            ORDER BY rank DESC
            LIMIT $2
            "#,
        )
        .bind(search_term)
        .bind(limit)
        .fetch_all(pool.get_ref())
        .await?;
```

- [ ] **Step 3: Update the company search query**

Same pattern but filtering on `company_id`:

```rust
        companies = sqlx::query_as::<_, SearchResult>(
            r#"
            SELECT c.id, c.name,
                   ts_rank(c.search_vector, plainto_tsquery('english', $1)) as rank,
                   rs.avg_rating,
                   COALESCE(rs.review_count, 0) as review_count,
                   rs.would_return_pct
            FROM companies c
            LEFT JOIN LATERAL (
                SELECT AVG(rating)::float8 as avg_rating,
                       COUNT(*) as review_count,
                       COUNT(*) FILTER (WHERE would_return = true) * 100.0 / NULLIF(COUNT(*), 0) as would_return_pct
                FROM reviews WHERE company_id = c.id
            ) rs ON true
            WHERE c.search_vector @@ plainto_tsquery('english', $1)
            ORDER BY rank DESC
            LIMIT $2
            "#,
        )
        .bind(search_term)
        .bind(limit)
        .fetch_all(pool.get_ref())
        .await?;
```

- [ ] **Step 4: Remove user search from `SearchResult`**

Users don't have ratings, so create a separate struct for user results or keep using the old struct. Simplest approach — create a `UserSearchResult`:

```rust
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UserSearchResult {
    pub id: Uuid,
    pub name: String,
    pub rank: f32,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub events: Vec<SearchResult>,
    pub companies: Vec<SearchResult>,
    pub users: Vec<UserSearchResult>,
    pub total: i64,
}
```

- [ ] **Step 5: Verify compilation**

Run: `cd backend && cargo check`
Expected: Compiles without errors.

- [ ] **Step 6: Commit**

```bash
git add backend/src/routes/search.rs
git commit -m "feat(api): enhance search results with ratings and would_return_pct"
```

---

### Task 11: Implement compare endpoint

**Files:**
- Modify: `backend/src/routes/compare.rs`

- [ ] **Step 1: Replace placeholder with full implementation**

```rust
use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::models::review::{CategoryAvg, RatingDistribution};

#[derive(Debug, serde::Deserialize)]
pub struct CompareParams {
    #[serde(rename = "type")]
    pub entity_type: String,
    pub ids: String,
}

#[derive(Debug, serde::Serialize)]
pub struct CompareEntity {
    pub id: Uuid,
    pub name: String,
    pub avg_rating: Option<f64>,
    pub review_count: i64,
    pub would_return_pct: Option<f64>,
    pub category_ratings: Vec<CategoryAvg>,
}

/// GET /api/compare?type=company&ids=uuid1,uuid2
#[get("/compare")]
pub async fn compare(
    pool: web::Data<PgPool>,
    query: web::Query<CompareParams>,
) -> Result<HttpResponse, ApiError> {
    let ids: Vec<Uuid> = query
        .ids
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    if ids.len() < 2 || ids.len() > 4 {
        return Err(ApiError::BadRequest(
            "Provide 2-4 comma-separated UUIDs".to_string(),
        ));
    }

    let mut entities = Vec::new();

    for entity_id in &ids {
        let (name, stats, would_return, category_ratings) = match query.entity_type.as_str() {
            "company" => {
                let name: Option<(String,)> =
                    sqlx::query_as("SELECT name FROM companies WHERE id = $1")
                        .bind(entity_id)
                        .fetch_optional(pool.get_ref())
                        .await?;
                let name = name
                    .ok_or_else(|| ApiError::NotFound(format!("Company {} not found", entity_id)))?
                    .0;

                let stats: (Option<f64>, i64) = sqlx::query_as(
                    "SELECT AVG(rating)::float8, COUNT(*) FROM reviews WHERE company_id = $1",
                )
                .bind(entity_id)
                .fetch_one(pool.get_ref())
                .await?;

                let wr: (Option<f64>,) = sqlx::query_as(
                    "SELECT COUNT(*) FILTER (WHERE would_return = true) * 100.0 / NULLIF(COUNT(*), 0) FROM reviews WHERE company_id = $1",
                )
                .bind(entity_id)
                .fetch_one(pool.get_ref())
                .await?;

                let cats = sqlx::query_as::<_, CategoryAvg>(
                    r#"
                    SELECT rr.category, AVG(rr.score)::float8 as avg
                    FROM review_ratings rr
                    JOIN reviews r ON r.id = rr.review_id
                    WHERE r.company_id = $1
                    GROUP BY rr.category
                    "#,
                )
                .bind(entity_id)
                .fetch_all(pool.get_ref())
                .await?;

                (name, stats, wr, cats)
            }
            "event" => {
                let name: Option<(String,)> =
                    sqlx::query_as("SELECT name FROM events WHERE id = $1")
                        .bind(entity_id)
                        .fetch_optional(pool.get_ref())
                        .await?;
                let name = name
                    .ok_or_else(|| ApiError::NotFound(format!("Event {} not found", entity_id)))?
                    .0;

                let stats: (Option<f64>, i64) = sqlx::query_as(
                    "SELECT AVG(rating)::float8, COUNT(*) FROM reviews WHERE event_id = $1",
                )
                .bind(entity_id)
                .fetch_one(pool.get_ref())
                .await?;

                let wr: (Option<f64>,) = sqlx::query_as(
                    "SELECT COUNT(*) FILTER (WHERE would_return = true) * 100.0 / NULLIF(COUNT(*), 0) FROM reviews WHERE event_id = $1",
                )
                .bind(entity_id)
                .fetch_one(pool.get_ref())
                .await?;

                let cats = sqlx::query_as::<_, CategoryAvg>(
                    r#"
                    SELECT rr.category, AVG(rr.score)::float8 as avg
                    FROM review_ratings rr
                    JOIN reviews r ON r.id = rr.review_id
                    WHERE r.event_id = $1
                    GROUP BY rr.category
                    "#,
                )
                .bind(entity_id)
                .fetch_all(pool.get_ref())
                .await?;

                (name, stats, wr, cats)
            }
            _ => {
                return Err(ApiError::BadRequest(
                    "type must be 'event' or 'company'".to_string(),
                ));
            }
        };

        entities.push(CompareEntity {
            id: *entity_id,
            name,
            avg_rating: stats.0,
            review_count: stats.1,
            would_return_pct: would_return.0,
            category_ratings,
        });
    }

    Ok(HttpResponse::Ok().json(entities))
}
```

- [ ] **Step 2: Verify compilation**

Run: `cd backend && cargo check`
Expected: Compiles without errors.

- [ ] **Step 3: Commit**

```bash
git add backend/src/routes/compare.rs
git commit -m "feat(api): implement compare endpoint for side-by-side entity comparison"
```

---

## Chunk 5: Integration Verification + Docs

### Task 12: Full compilation and manual smoke test

- [ ] **Step 1: Clean build**

Run: `cd backend && cargo build`
Expected: Clean build with no errors.

- [ ] **Step 2: Run the backend**

Run: `cd backend && cargo run`
Expected: Server starts on `:8080`.

- [ ] **Step 3: Smoke test tag endpoints**

```bash
# Create a tag
curl -s -X POST http://localhost:8080/api/tags \
  -H 'Content-Type: application/json' \
  -d '{"name": "well-organized"}' | jq .

# List tags
curl -s http://localhost:8080/api/tags | jq .

# Create duplicate (should return existing)
curl -s -X POST http://localhost:8080/api/tags \
  -H 'Content-Type: application/json' \
  -d '{"name": "Well-Organized"}' | jq .
```

- [ ] **Step 4: Smoke test enhanced search**

```bash
curl -s "http://localhost:8080/api/search?q=hack" | jq '.events[0] | keys'
```

Expected: Response includes `avg_rating`, `review_count`, `would_return_pct` fields.

- [ ] **Step 5: Smoke test compare (empty data is fine)**

```bash
# Will return 404 for non-existent IDs, confirming the endpoint is wired up
curl -s "http://localhost:8080/api/compare?type=event&ids=00000000-0000-0000-0000-000000000001,00000000-0000-0000-0000-000000000002" | jq .
```

---

### Task 13: Update documentation

**Files:**
- Modify: `CLAUDE.md`
- Modify: `README.md`
- Modify: `TODO.md`
- Modify: `CHANGELOG.md`

- [ ] **Step 1: Update CLAUDE.md**

Add to the "Database Setup" section:
```
psql -d ratemyhackathons -f backend/migrations/20260314_rmp_ratings.sql
```

Add to "Key Tables" description:
```
`review_ratings` stores per-category scores (1-5) for each review across 10 categories. `tags` and `review_tags` enable crowd-sourced labeling. Reviews can now target either events OR companies (XOR constraint).
```

Add to "Backend Structure" or "API Changes":
```
New route modules: `tags` (CRUD for crowd-sourced tags) and `compare` (side-by-side entity comparison). Enhanced endpoints: search returns `avg_rating`/`review_count`/`would_return_pct`, company and event detail return `category_ratings`, `top_tags`, `rating_distribution`, per-review category breakdowns.
```

- [ ] **Step 2: Update README.md**

Add new API endpoints to the API reference section and update the schema description.

- [ ] **Step 3: Update TODO.md**

Mark Phase 1 tasks as complete. Add Phase 2 (Clerk Auth) as next.

- [ ] **Step 4: Update CHANGELOG.md**

Add entry:
```
## [Unreleased] - 2026-03-14

### Added
- Multi-dimensional rating system with 10 hackathon-specific categories (organization, prizes, mentorship, judging, venue, food, swag, networking, communication, vibes)
- `review_ratings` table for per-category scores (1-5)
- Tag system with `tags` and `review_tags` tables
- Company reviews: reviews can now target companies (not just events) with XOR constraint
- "Would return" boolean metric on reviews
- Tag CRUD endpoints: GET /api/tags, GET /api/tags/top, POST /api/tags
- Compare endpoint: GET /api/compare for side-by-side entity comparison
- Enhanced search results with avg_rating, review_count, would_return_pct
- Enhanced company detail with category_ratings, top_tags, rating_distribution, reviews
- Enhanced event detail with category_ratings, top_tags, rating_distribution
- Per-review category ratings and tags in review detail endpoint
- clerk_id column on users table (prep for Phase 2 auth)
```

- [ ] **Step 5: Commit**

```bash
git add CLAUDE.md README.md TODO.md CHANGELOG.md
git commit -m "docs: update all docs for Phase 1 multi-dimensional ratings"
```
