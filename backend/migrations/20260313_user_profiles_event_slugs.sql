-- Add user profile fields
-- Migration: 20260313_user_profiles.sql

-- ═══════════════════════════════════════════════
-- User profile fields
-- ═══════════════════════════════════════════════

ALTER TABLE users
    ADD COLUMN display_name TEXT,         -- full display name
    ADD COLUMN bio TEXT,                  -- short bio
    ADD COLUMN age INT CHECK (age >= 13 AND age <= 150),
    ADD COLUMN github TEXT,              -- github.com/username
    ADD COLUMN twitter TEXT,             -- twitter.com/handle
    ADD COLUMN linkedin TEXT,            -- linkedin.com/in/slug
    ADD COLUMN website TEXT;             -- personal website
