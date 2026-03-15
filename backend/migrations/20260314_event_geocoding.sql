-- Add geocoding columns to events for globe visualization
ALTER TABLE events ADD COLUMN latitude DOUBLE PRECISION;
ALTER TABLE events ADD COLUMN longitude DOUBLE PRECISION;

CREATE INDEX idx_events_geo ON events (latitude, longitude) WHERE latitude IS NOT NULL;
