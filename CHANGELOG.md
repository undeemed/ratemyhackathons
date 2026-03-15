# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Common Changelog](https://common-changelog.org/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Frontend** (`frontend/`) — SvelteKit + Svelte 5 + Tailwind v4 + bun
  - cobe WebGL globe with hackathon location dots on hero section
  - GSAP ScrollTrigger storyboard landing page (7 sections: hero, stats, trending, how-it-works, companies, testimonials, CTA)
  - Full inner pages: events (list + detail), companies (list + detail), users, search with tabs
  - Dark theme (#0a0a0f base, indigo accents) matching analytics dashboard
  - Typed API client (`lib/api.ts`) + TypeScript interfaces
  - Reusable components: Globe, EventCard, ReviewCard, Nav, Footer

- **Event geocoding**
  - DB migration: `latitude`/`longitude` columns on events table
  - Backend: lat/lng in all event models + `GET /api/events/globe` endpoint
  - Crawler: Nominatim geocoding module (`geocode.py`) with 50-city seed cache

- **CLAUDE.md** — context7 MCP requirement, frontend commands, updated architecture

### Planned

---

- **API Reconnaissance** (`services/crawler/cv/API_RECON.md`)
  - Discovered Lu.ma open REST API: `api.lu.ma/discover/get-paginated-events` — no auth, rich JSON, cursor pagination
  - **Cracked Cerebral Valley public API**: `api.cerebralvalley.ai/v1/public/event/pull` — no auth required!
    - Extracted from 37 JS bundles via AIDA Exegol probing
    - Also found: `POST /v1/search/event/search` (keyword + vector search)
  - Both platforms return full event data (name, dates, location, description, image) with zero authentication

- **Cerebral Valley spider** (`services/crawler/spiders/cerebralvalley.py`)
  - Fetches events from public API: `GET /v1/public/event/pull?{status}=true`
  - Pulls both `featured` and `approved` status endpoints, deduplicates by event ID
  - Dry run: 9,258 events extracted

- **Luma spider** (`services/crawler/spiders/luma.py`)
  - Fetches events from public API: `GET api.lu.ma/discover/get-paginated-events`
  - Cursor-based pagination, up to 20 pages (1,000 events max per run)
  - Dry run: 920 events extracted

- **Standalone dry-run script** (`services/crawler/dry_run.py`)
  - Test CV and Luma spiders without a database connection
  - Usage: `python dry_run.py cv`, `python dry_run.py luma`, or both

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
- **Sponsor scraper** (`services/crawler/sponsors.py`)
  - 4-strategy extraction: CSS class/id, heading detection, src path, LLM fallback
  - Uses StealthyFetcher (Playwright) for JS-rendered event pages
  - Smart text extraction for LLM — finds sponsor keywords and grabs surrounding context
- **OpenRouter LLM module** (`services/crawler/llm.py`)
  - Dynamic free model discovery from `/api/v1/models`
  - Provider-priority rotation (Google first)
  - Paid model fallback (`gemini-2.0-flash`) when free tier is congested
- `TODO.md` for project-wide task tracking
- `.agent/rules.md` — agent rules requiring doc updates after every run
