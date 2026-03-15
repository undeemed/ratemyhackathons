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
psql -d ratemyhackathons -f backend/migrations/20260314_rmp_ratings.sql
```

## Architecture

### Data Flow
```
Spiders (scrape + geocode) → main.py (dedup + process) → PostgreSQL ← Backend API (serve) ← Frontend (SvelteKit)
                                                              ↑
                                                        Analytics API → SSE → Dashboard
```

### Frontend Architecture
SvelteKit app at `frontend/`. Editorial brutalist B&W design: pure black (`#000`) background, white text, `Instrument Serif` italic for display, `Space Mono` monospace for body. Grain texture overlay via SVG feTurbulence. No border-radius anywhere.

Landing page is a storyboard with 7 scroll-triggered sections (hero, marquee, stats, featured events, how-it-works, pull quote, CTA) powered by GSAP ScrollTrigger. Hero has an interactive cobe WebGL globe with dots at hackathon locations (lat/lng from DB) — globe starts shifted right at 90% viewport width, then morphs to center during scroll-driven showcase. Date-based marker brightness: closer events glow brighter, distant future events are dimmer. Featured events use a flat 3-column grid layout.

**Globe component (`Globe.svelte`) critical patterns:**
- Wrapper uses `aspect-square w-full`, canvas uses `w-full aspect-square` (NOT `h-full` — CSS `height: 100%` can't resolve against `aspect-ratio`-derived parent height, causing oval rendering)
- Cobe width cached via `resize` event listener (official cobe pattern — never read `offsetWidth` inside `onRender` to avoid forced reflows). Same cached `width` used for BOTH `state.width` and `state.height`.
- Cobe creation wrapped in `requestAnimationFrame` with `destroyed` guard (Svelte mounts children before parents — Globe's `onMount` fires before page's `onMount` sets GSAP container dimensions)
- `devicePixelRatio` capped at 2 (`Math.min(window.devicePixelRatio || 1, 2)`)
- Per-marker `color` array for date-based brightness: `markerBrightness()` returns 0.4–1.0 for future events (closer = brighter), 0.3–0.7 for past events (fading)
- `prevMarkerRef` tracking avoids reassigning `state.markers` every frame when derived array hasn't changed
- Dynamic `mapSamples`: 8K when focused (showcase), 16K for hero (high-res)
- Cleanup: `destroyed` flag prevents `onRender` after navigation, `cancelAnimationFrame` + `removeEventListener('resize')` + `globe.destroy()`

**Landing page scroll animation (`+page.svelte`):**
- All GSAP wrapped in `gsap.context()` — single `ctx.revert()` on unmount cleans up everything
- Hero globe: 90vw on desktop (max 1440px), positioned at `top: 45%` shifted right (`vw * 1.05 - heroW`)
- Morph uses ONLY transform properties (`x`, `y`, `scale`) — no `width`/`height`/`left` changes during scroll, avoiding forced layout reflows. `morphScale = maxShowSize / heroW`, `morphX = vw/2 - heroCenterX`
- `scrub: 2` (no snap — snap caused lag on reverse scroll)
- `globeFocus` is a plain mutable object `{lat: 0, lng: 0}` — GSAP tweens it directly, Globe reads it every frame in `onRender`. When both are 0, globe auto-spins.
- Phase sequence: hero hold → morph (fade text, translate+scale globe to center) → cycle events (spin globe to each lat/lng, slide cards in/out) → exit fade

Theme colors in `app.css` via Tailwind v4 `@theme`: `bg #000`, `surface #080808`, `elevated #141414`, `border #2a2a2a`, `text #fff`, `muted #999`, `dim #555`, `accent #e0e0e0`.

All page loaders have `.catch()` fallback — landing page has hardcoded demo events, inner pages show empty states. Vite proxies `/api` to backend at `:8080`.

### Backend Structure
Routes are registered under `/api` scope in `main.rs`. Each route module (`routes/*.rs`) contains Actix handler functions with `#[get]`/`#[post]` attributes. Models (`models/*.rs`) are DTOs with Serde derive. SQL uses correlated subqueries (not N+1) for list endpoints. IDs are UUIDv7. Input sanitized with `ammonia`, validated with `validator`. Route modules: `events`, `companies`, `users`, `reviews`, `search`, `tags` (CRUD for crowd-sourced tags), `compare` (side-by-side entity comparison). Enhanced endpoints: search returns `avg_rating`/`review_count`/`would_return_pct`, company and event detail return `category_ratings`, `top_tags`, `rating_distribution`, per-review category breakdowns.

### Auth Architecture
Clerk-based authentication. Frontend uses `svelte-clerk`: `withClerkHandler()` in `hooks.server.ts`, `buildClerkProps()` in `+layout.server.ts`, `<ClerkProvider>` wrapper in `+layout.svelte`. Sign-in/sign-up pages at `/sign-in` and `/sign-up`. Nav shows `<Show when="signed-in">` / `<Show when="signed-out">` for auth-aware UI.

Backend uses `auth.rs` module with JWKS-based JWT verification (`jsonwebtoken` crate). `AuthState` fetches and caches Clerk's JWKS endpoint (1-hour TTL), verifies RS256 JWT signatures, extracts `sub` claim. `sync_clerk_user()` does lookup-or-create by `clerk_id` in `users` table. Mutating endpoints (POST) accept `Option<web::Data<AuthState>>` — when `CLERK_JWKS_URL` is set, auth is required; when not set (dev mode), falls back to `user_id` in request body. GET endpoints remain public.

### Crawler Architecture
- **Spiders** (`spiders/*.py`) — Source-specific scrapers returning `list[dict]`. Use `Fetcher` for static HTML (MLH), `StealthyFetcher` for JS-rendered SPAs (Hackiterate), or direct HTTP for APIs (CV, Luma).
- **Dedup** (`dedup.py`) — Two-layer: SHA-256 hash on `(source_type, url)` for exact dedup, Levenshtein similarity (≥0.85) for fuzzy name matching. Cross-source dedup normalizes URLs (strips UTM params, normalizes luma.com→lu.ma).
- **Geocoding** (`geocode.py`) — Nominatim via geopy with local cache. Populates lat/lng on event insert.
- **Sponsors** (`sponsors.py`) — 4-strategy extraction: CSS class/id selectors, heading-based section scan, image src path matching, LLM fallback via OpenRouter.
- Source priority for dedup merging: luma > cerebralvalley > mlh > hackiterate (Luma has richest data).

### Key Tables
`events` (with `latitude`/`longitude`), `companies`, `users`, `reviews` are the core entities. `event_companies` is the many-to-many join. `crawl_sources` tracks provenance with `source_hash` for dedup. `scrape_sources` is the crawler registry (enabled sources, poll intervals). `review_ratings` stores per-category scores (1-5) for each review across 10 categories (organization, prizes, mentorship, judging, venue, food, swag, networking, communication, vibes). `tags` and `review_tags` enable crowd-sourced labeling. Reviews can target either events OR companies (XOR constraint). Full schema in README.md.

## Environment Variables

**Backend** (`backend/.env`): `DATABASE_URL`, `HOST`, `PORT`, `CLERK_JWKS_URL` (optional), `CLERK_ISSUER` (optional)

**Frontend** (`frontend/.env`): `PUBLIC_CLERK_PUBLISHABLE_KEY`, `CLERK_SECRET_KEY`

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

## Self-Learning

1. **Read `tasks/lessons.md` at the start of every session.** Apply all rules before touching any code.
2. **After any correction from the user**, append a row to `tasks/lessons.md`: `| date | what went wrong | rule for next time |`
3. Never repeat a mistake that already has a lesson entry.
