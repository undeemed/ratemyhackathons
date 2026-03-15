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
