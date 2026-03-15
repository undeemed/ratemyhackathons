-- Add review votes (helpful/unhelpful) and threaded review comments
-- Migration: 20260313_review_votes_comments.sql

-- ═══════════════════════════════════════════════
-- Tables
-- ═══════════════════════════════════════════════

-- Helpful/Unhelpful votes on reviews (one vote per user per review)
CREATE TABLE review_votes (
    id          UUID PRIMARY KEY,
    review_id   UUID NOT NULL REFERENCES reviews(id) ON DELETE CASCADE,
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    helpful     BOOLEAN NOT NULL,  -- true = helpful, false = unhelpful
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (review_id, user_id)    -- one vote per user per review
);

-- Threaded comments on reviews (Reddit-style nesting)
CREATE TABLE review_comments (
    id                UUID PRIMARY KEY,
    review_id         UUID NOT NULL REFERENCES reviews(id) ON DELETE CASCADE,
    user_id           UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    parent_comment_id UUID REFERENCES review_comments(id) ON DELETE CASCADE,  -- NULL = top-level comment
    body              TEXT NOT NULL,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ═══════════════════════════════════════════════
-- Indexes
-- ═══════════════════════════════════════════════

CREATE INDEX idx_review_votes_review_id ON review_votes (review_id);
CREATE INDEX idx_review_votes_user_id ON review_votes (user_id);
CREATE INDEX idx_review_comments_review_id ON review_comments (review_id);
CREATE INDEX idx_review_comments_user_id ON review_comments (user_id);
CREATE INDEX idx_review_comments_parent ON review_comments (parent_comment_id);
CREATE INDEX idx_review_comments_created_at ON review_comments (review_id, created_at);
