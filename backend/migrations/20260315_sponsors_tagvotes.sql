-- RateMyHackathons: event sponsors table + tag voting
-- Depends on: 20260314_rmp_ratings.sql

-- ═══════════════════════════════════════════════
-- 1. Event sponsors (scraped from event pages)
-- ═══════════════════════════════════════════════

CREATE TABLE event_sponsors (
    id       UUID PRIMARY KEY,
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    name     TEXT NOT NULL,
    logo_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_event_sponsors_event ON event_sponsors(event_id);
CREATE UNIQUE INDEX idx_event_sponsors_unique ON event_sponsors(event_id, LOWER(name));

-- ═══════════════════════════════════════════════
-- 2. Tag votes (upvote tags on reviews)
-- ═══════════════════════════════════════════════

CREATE TABLE tag_votes (
    id      UUID PRIMARY KEY,
    tag_id  UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tag_id, user_id)
);

CREATE INDEX idx_tag_votes_tag ON tag_votes(tag_id);
CREATE INDEX idx_tag_votes_user ON tag_votes(user_id);
