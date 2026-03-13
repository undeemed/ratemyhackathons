# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Common Changelog](https://common-changelog.org/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial backend scaffold with Rust/Actix Web
- PostgreSQL schema with full-text search (tsvector/tsquery)
- REST API endpoints: events, companies, users, reviews, search
- Many-to-many event ↔ company relationships
- Paginated list endpoints with filtering
- Full-text search across events, companies, and users
- Crawler metadata table (`crawl_sources`) with JSONB storage
- Database migration for initial schema
- Review votes (helpful/unhelpful) with upsert
- Threaded review comments (nested replies via `parent_comment_id`)
- User profiles (age, bio, social links: GitHub, Twitter, LinkedIn, website)
- Event quick links (`url` field pointing to external event pages)
- Input validation via `validator` crate on all create DTOs
- HTML sanitization via `ammonia` crate on all free-text fields
- **Crawler service** (`services/crawler/`) — Python + Scrapling
  - MLH spider (server-rendered HTML scraping)
  - Hackiterate spider (directory listing scraping)
  - Rotating residential proxy support via `ProxyRotator`
  - SHA-256 hash deduplication + Levenshtein fuzzy name matching
  - Best-effort company detection from event text
  - `scrape_sources` table for dynamic source registry
  - CLI: `--once`, `--daemon`, `--dry-run` modes
- **Analytics service** (`services/analytics/`) — Rust + SvelteKit
  - Rust API on `:8081` with crawl stats, trending, timeline, rating distribution
  - SSE live feed for real-time crawl + review events
  - SvelteKit dashboard on `:5174` with Tailwind v4
  - 6 dashboard panels: stat cards, source health, live feed, ratings, trending, recent reviews
- Sub-READMEs with Mermaid UML diagrams for backend, services, crawler, analytics
- Documentation links table in root README
