-- Registered scrape sources (add/enable/disable dynamically)
CREATE TABLE scrape_sources (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name                TEXT NOT NULL,
    source_type         TEXT NOT NULL,       -- "mlh", "hackiterate", "luma", "cerebralvalley"
    base_url            TEXT NOT NULL,
    enabled             BOOLEAN NOT NULL DEFAULT true,
    poll_interval_hours INT NOT NULL DEFAULT 6,
    last_polled_at      TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Dedup hash on crawl_sources
ALTER TABLE crawl_sources ADD COLUMN source_hash TEXT;
CREATE UNIQUE INDEX idx_crawl_sources_hash ON crawl_sources (source_hash);

-- Seed initial sources
INSERT INTO scrape_sources (name, source_type, base_url) VALUES
    ('MLH 2026', 'mlh', 'https://mlh.com/seasons/2026/events'),
    ('MLH 2025', 'mlh', 'https://mlh.com/seasons/2025/events'),
    ('Hackiterate', 'hackiterate', 'https://hackiterate.com/directory');
