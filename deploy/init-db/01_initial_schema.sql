-- RateMyHackathons initial schema
-- PostgreSQL with full-text search support


-- ═══════════════════════════════════════════════
-- Tables
-- ═══════════════════════════════════════════════

CREATE TABLE events (
    id          UUID PRIMARY KEY,
    name        TEXT NOT NULL,
    description TEXT,
    location    TEXT,
    url         TEXT,
    start_date  DATE,
    end_date    DATE,
    image_url   TEXT,
    search_vector TSVECTOR,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE companies (
    id          UUID PRIMARY KEY,
    name        TEXT NOT NULL,
    logo_url    TEXT,
    website     TEXT,
    description TEXT,
    search_vector TSVECTOR,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE users (
    id          UUID PRIMARY KEY,
    username    TEXT NOT NULL UNIQUE,
    email       TEXT NOT NULL UNIQUE,
    avatar_url  TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE event_companies (
    event_id    UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    company_id  UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    role        TEXT,  -- e.g., 'sponsor', 'organizer', 'partner'
    PRIMARY KEY (event_id, company_id)
);

CREATE TABLE reviews (
    id          UUID PRIMARY KEY,
    event_id    UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    rating      INT NOT NULL CHECK (rating >= 1 AND rating <= 5),
    title       TEXT,
    body        TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (event_id, user_id)  -- one review per user per event
);

CREATE TABLE crawl_sources (
    id          UUID PRIMARY KEY,
    event_id    UUID REFERENCES events(id) ON DELETE SET NULL,
    source_url  TEXT NOT NULL,
    source_type TEXT,  -- e.g., 'devpost', 'mlh', 'manual'
    raw_data    JSONB,
    crawled_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ═══════════════════════════════════════════════
-- Indexes
-- ═══════════════════════════════════════════════

CREATE INDEX idx_events_search ON events USING GIN (search_vector);
CREATE INDEX idx_companies_search ON companies USING GIN (search_vector);
CREATE INDEX idx_events_start_date ON events (start_date);
CREATE INDEX idx_reviews_event_id ON reviews (event_id);
CREATE INDEX idx_reviews_user_id ON reviews (user_id);
CREATE INDEX idx_event_companies_company_id ON event_companies (company_id);
CREATE INDEX idx_crawl_sources_event_id ON crawl_sources (event_id);

-- ═══════════════════════════════════════════════
-- Full-text search triggers
-- ═══════════════════════════════════════════════

-- Auto-update search_vector on events
CREATE OR REPLACE FUNCTION events_search_vector_update() RETURNS TRIGGER AS $$
BEGIN
    NEW.search_vector :=
        setweight(to_tsvector('english', COALESCE(NEW.name, '')), 'A') ||
        setweight(to_tsvector('english', COALESCE(NEW.description, '')), 'B') ||
        setweight(to_tsvector('english', COALESCE(NEW.location, '')), 'C');
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER events_search_vector_trigger
    BEFORE INSERT OR UPDATE ON events
    FOR EACH ROW EXECUTE FUNCTION events_search_vector_update();

-- Auto-update search_vector on companies
CREATE OR REPLACE FUNCTION companies_search_vector_update() RETURNS TRIGGER AS $$
BEGIN
    NEW.search_vector :=
        setweight(to_tsvector('english', COALESCE(NEW.name, '')), 'A') ||
        setweight(to_tsvector('english', COALESCE(NEW.description, '')), 'B');
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER companies_search_vector_trigger
    BEFORE INSERT OR UPDATE ON companies
    FOR EACH ROW EXECUTE FUNCTION companies_search_vector_update();

-- Auto-update updated_at on events
CREATE OR REPLACE FUNCTION update_updated_at() RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER events_updated_at_trigger
    BEFORE UPDATE ON events
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
