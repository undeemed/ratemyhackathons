# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Common Changelog](https://common-changelog.org/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Internal Admin Dashboard** (`services/analytics/dashboard/`)
  - Multi-page layout with sidebar navigation (Overview, Events, Crawl Sources, Reviews)
  - Overview page: entity counts from main API, crawl activity stats, live SSE feed, trending events, rating distribution, recent reviews
  - Events browser: paginated table with client-side search/filter, color-coded ratings, company tags
  - Crawl Sources page: source health table with status indicators, by-type breakdown, 30-day crawl history bar chart
  - Reviews page: total/average stats, rating distribution with %, most-reviewed ranking, recent reviews table
  - Vite proxy: `/main-api` → main backend `:8080` for event/company/user data alongside analytics API `:8081`

- **Hero UX improvements**
  - "rated." text uses red → yellow → green gradient matching score color system
  - Search mode toggle: Hackathons / Companies buttons swap placeholder and search type
  - "Browse by company" and "All events" links below search bar
  - WIP popup shows once only (persists dismiss to localStorage), button text "Enter anyways"
  - Recent events section: flat 3-column grid replacing oversized magazine layout with `row-span-2`

- **Globe & Landing Page Overhaul**
  - Fixed oval globe rendering: canvas uses `w-full aspect-square` (CSS `height: 100%` can't resolve against `aspect-ratio`-derived height)
  - Cobe uses `offsetWidth` for both width and height (official cobe pattern for circular rendering)
  - Cobe creation delayed via `requestAnimationFrame` (Svelte mounts children before parents)
  - Date-based marker brightness: closer/ongoing events glow brighter, distant future events are dimmer
  - Globe surface tuned (`mapBrightness: 2`, `baseColor: [0.2,0.2,0.2]`) so hackathon markers stand out
  - Marker dots enlarged to `size: 0.1` with per-marker color based on `start_date`
  - Scroll-driven showcase: hero globe (shifted right) morphs to centered showcase (80% size, scale 1.25)
  - Globe zoom applied once at morph start, stays zoomed during event cycling, zooms out at exit
  - Globe centering via `left: '50%'` + GSAP `xPercent: -50` for reliable centering with scale transforms
  - Hero search bar with hackathons/companies toggle and gradient "rated." text

- **Frontend Rebuild — RMP-Style UI (Phase 3+4)**
  - Score color system: green (#4caf50) ≥4.0, yellow (#ffc107) ≥3.0, red (#ef5350) <3.0
  - Theme vars: `--color-score-green`, `--color-score-yellow`, `--color-score-red`
  - 5 new components: ScoreBadge (color-coded score square), CategoryGrid (2-column category bars), RatingDistribution (Awesome→Awful bar chart), TagPills (tag badges), RatingSlider (1-5 selector)
  - Updated ReviewCard: per-category score breakdown, would-return badge
  - RMP-style search results: score badge + name + would-return % in list layout
  - Company detail page: giant score badge, category grid, top tags, rating distribution chart, events list, reviews with breakdowns
  - Event detail page: same structure with image, dates, companies, rate/compare CTAs
  - Rate form for events (`/events/{id}/rate`): auth gate via Clerk `<Show>`, 10 category sliders, would-return toggle, tag selector, 350-5000 char review body
  - Rate form for companies (`/companies/{id}/rate`): same structure
  - Compare page (`/compare`): side-by-side entity comparison with category bars, shareable URLs
  - Compare link added to Nav (desktop + mobile)
  - Updated `types.ts`: CategoryAvg, TagCount, RatingDistribution, CompanyDetail, CompareEntity, CreateReviewPayload, scoreColor/scoreLabel/scoreBg helpers, RATING_CATEGORIES constant
  - Updated `api.ts`: getCompany→CompanyDetail, compare(), listTags(), createTag(), search with type filter

- **Clerk Authentication (Phase 2)**
  - Frontend: `svelte-clerk` integration with `withClerkHandler()`, `buildClerkProps()`, `<ClerkProvider>`
  - Sign-in (`/sign-in`) and sign-up (`/sign-up`) pages styled to brutalist theme
  - Auth-aware Nav component with `<Show when="signed-in/out">` conditional rendering + `<UserButton />`
  - Backend: `auth.rs` module with JWKS-cached JWT verification (RS256, `jsonwebtoken` crate)
  - `AuthState` with 1-hour JWKS cache TTL, `sync_clerk_user()` for clerk_id → user lookup-or-create
  - `Unauthorized` error variant in `ApiError` enum
  - Graceful auth: when `CLERK_JWKS_URL` is set, JWT required for POST endpoints; when not set (dev), falls back to `user_id` in body
  - `user_id` made optional in `CreateReview`, `CreateReviewVote`, `CreateReviewComment` DTOs
  - Updated `create_review`, `vote_review`, `create_review_comment` handlers with `resolve_user_id()` helper
  - New dependencies: `jsonwebtoken` 9, `reqwest` 0.12 (backend); `svelte-clerk` 1.0 (frontend)
  - `.env.example` files for both backend and frontend with Clerk configuration

- **RMP-Style Multi-Dimensional Ratings (Phase 1: Schema + API)**
  - 10 hackathon-specific rating categories: organization, prizes, mentorship, judging, venue, food, swag, networking, communication, vibes
  - `review_ratings` table for per-category scores (1-5) per review
  - `tags` and `review_tags` tables for crowd-sourced labeling
  - Company reviews: reviews can now target companies (not just events) via XOR constraint
  - "Would return" boolean metric on reviews
  - Overall `rating` auto-computed as average of 10 category scores
  - Tag CRUD endpoints: `GET /api/tags`, `GET /api/tags/top`, `POST /api/tags`
  - Compare endpoint: `GET /api/compare?type=company&ids=uuid1,uuid2` for side-by-side comparison
  - Enhanced search results with `avg_rating`, `review_count`, `would_return_pct`
  - Enhanced company detail with `category_ratings`, `top_tags`, `rating_distribution`, per-review breakdowns
  - Enhanced event detail with same additions
  - Per-review category ratings and tags in review detail endpoint
  - `clerk_id` column on users table (prep for Phase 2 auth)
  - Partial unique indexes for one review per user per event/company

- **Frontend v2** — Editorial brutalist B&W redesign
  - Pure black/white color scheme with Instrument Serif italic + Space Mono monospace
  - Grain texture overlay (SVG feTurbulence), no border-radius
  - Improved contrast: borders `#2a2a2a`, muted `#999`, dim `#555`, accent `#e0e0e0`
  - EventCard redesign: corner accents, large italic rating numbers, hover accent line
  - Magazine layout: featured `row-span-2` + 2 stacked right cards + 3 bottom row
  - Alternating section backgrounds (bg/surface) for visual rhythm
  - CTA with filled primary button (white on black, inverts on hover)
  - Graceful API fallback: demo events on landing page, empty states on inner pages
  - All page loaders wrapped in try/catch — no more 500 errors without backend

- **Frontend v1** (`frontend/`) — SvelteKit + Svelte 5 + Tailwind v4 + bun
  - cobe WebGL globe with hackathon location dots on hero section
  - GSAP ScrollTrigger storyboard landing page (7 sections: hero, marquee, stats, featured events, how-it-works, pull quote, CTA)
  - Full inner pages: events (list + detail), companies (list + detail), users, search with tabs
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
