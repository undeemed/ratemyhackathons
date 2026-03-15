# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository. 
**CODEX WILL ALWAYS REVIEW YOUR WORK**

## Context7 MCP — Required

**Always use `mcp__plugin_context7_context7` (resolve-library-id → query-docs) before writing code that uses any library.** Look up docs for SvelteKit, Svelte 5, GSAP, cobe, Tailwind v4, geopy, or any other dependency before implementing. Do not rely on training data alone.

## Project Overview

RateMyHackathons is a platform for rating, reviewing, and discovering hackathon events. Four components share a PostgreSQL database:

- **Backend** (`backend/`) — Rust/Actix-Web REST API on `:8080`
- **Frontend** (`frontend/`) — SvelteKit + Svelte 5 (runes) + Tailwind v4 + cobe globe + GSAP on `:5173`
- **Crawler** (`services/crawler/`) — Python scraper (MLH, Hackiterate, Cerebral Valley, Luma)
- **Analytics** (`services/analytics/`) — Rust/Actix-Web API on `:8081` + SvelteKit dashboard on `:5174`

## Common Commands

### Frontend (SvelteKit)
```bash
cd frontend
bun install              # Install dependencies
bun dev                  # Dev server on :5173 (proxies /api to :8080)
bun run build            # Production build
bun run check            # Svelte type checking
```

### Backend (Rust)
```bash
cd backend
cargo run                    # Start API server on :8080
cargo test                   # Run all tests
cargo test test_name         # Run a single test
cargo test -- --nocapture    # Run tests with stdout visible
```

### Crawler (Python)
```bash
cd services/crawler
python dry_run.py                # Test all spiders without DB
python dry_run.py cv             # Test single spider (cv, luma, mlh, hackiterate)
python dry_run.py cv luma        # Test multiple spiders with cross-source dedup
python main.py --dry-run         # Full pipeline preview (needs DB)
python main.py --once            # Single crawl pass (needs DB)
python main.py --daemon          # Continuous polling (needs DB)
```

### Analytics
```bash
cd services/analytics
cargo run                              # API on :8081
cd dashboard && bun install && bun dev # Dashboard on :5174
```

### Database Setup
```bash
createdb ratemyhackathons
psql -d ratemyhackathons -f backend/migrations/20260313_initial_schema.sql
psql -d ratemyhackathons -f backend/migrations/20260313_review_votes_comments.sql
psql -d ratemyhackathons -f backend/migrations/20260313_user_profiles_event_slugs.sql
psql -d ratemyhackathons -f backend/migrations/20260313_crawl_registry.sql
psql -d ratemyhackathons -f backend/migrations/20260314_event_geocoding.sql
```

## Architecture

### Data Flow
```
Spiders (scrape + geocode) → main.py (dedup + process) → PostgreSQL ← Backend API (serve) ← Frontend (SvelteKit)
                                                              ↑
                                                        Analytics API → SSE → Dashboard
```

### Frontend Architecture
SvelteKit app at `frontend/`. Landing page is a storyboard with 7 scroll-triggered sections powered by GSAP ScrollTrigger. Hero section has an interactive cobe WebGL globe with dots at hackathon locations (lat/lng from DB). Dark theme matching analytics dashboard. Vite proxies `/api` to backend at `:8080`.

### Backend Structure
Routes are registered under `/api` scope in `main.rs`. Each route module (`routes/*.rs`) contains Actix handler functions with `#[get]`/`#[post]` attributes. Models (`models/*.rs`) are DTOs with Serde derive. SQL uses correlated subqueries (not N+1) for list endpoints. IDs are UUIDv7. Input sanitized with `ammonia`, validated with `validator`.

### Crawler Architecture
- **Spiders** (`spiders/*.py`) — Source-specific scrapers returning `list[dict]`. Use `Fetcher` for static HTML (MLH), `StealthyFetcher` for JS-rendered SPAs (Hackiterate), or direct HTTP for APIs (CV, Luma).
- **Dedup** (`dedup.py`) — Two-layer: SHA-256 hash on `(source_type, url)` for exact dedup, Levenshtein similarity (≥0.85) for fuzzy name matching. Cross-source dedup normalizes URLs (strips UTM params, normalizes luma.com→lu.ma).
- **Geocoding** (`geocode.py`) — Nominatim via geopy with local cache. Populates lat/lng on event insert.
- **Sponsors** (`sponsors.py`) — 4-strategy extraction: CSS class/id selectors, heading-based section scan, image src path matching, LLM fallback via OpenRouter.
- Source priority for dedup merging: luma > cerebralvalley > mlh > hackiterate (Luma has richest data).

### Key Tables
`events` (with `latitude`/`longitude`), `companies`, `users`, `reviews` are the core entities. `event_companies` is the many-to-many join. `crawl_sources` tracks provenance with `source_hash` for dedup. `scrape_sources` is the crawler registry (enabled sources, poll intervals). Full schema in README.md.

## Environment Variables

**Backend** (`backend/.env`): `DATABASE_URL`, `HOST`, `PORT`

**Crawler** (`services/crawler/.env`): `DATABASE_URL`, `PROXY_URL`, `OPENROUTER_API_KEY`

## Conventions

- **Always use bun** (never npm) for all JS/TS projects
- **Always use context7 MCP** to look up library docs before writing code
- Never overwrite `.env` files
- Use `uv` for Python package management in `services/crawler/`
- All crawler spiders use Scrapling (`scrapling.fetchers`) — `StealthyFetcher` for SPAs, `Fetcher` for static HTML
- Test spider changes with `dry_run.py` or sandbox scripts in `/tmp/` before modifying production code
- Rust edition 2024 (requires Rust 1.85+)
- After significant changes, update `TODO.md`, `CHANGELOG.md`, `README.md`, and relevant service READMEs
