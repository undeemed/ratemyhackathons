# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Common Changelog](https://common-changelog.org/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Homepage autocomplete search bar**
  - New `SearchAutocomplete.svelte` component replacing the static search form in the hero section
  - 300ms debounced search calls existing `GET /api/search` endpoint filtered by type (event/company)
  - Live dropdown shows up to 6 results with color-coded score badges, review counts, and entity type labels
  - Keyboard navigation: Arrow Up/Down to select, Enter to navigate, Escape to close
  - Click result navigates directly to event/company detail page
  - Enter with no selection falls through to `/search?q=...` results page
  - "View all results" link at bottom of dropdown
  - Inherits hero Hackathons/Companies toggle for filtered results

- **Tag voting (upvote tags on reviews)**
  - `tag_votes` table with `UNIQUE(tag_id, user_id)` constraint — one vote per user per tag
  - `POST /api/tags/{id}/vote` toggle endpoint — inserts vote if not exists, deletes if already voted; returns `{ voted, vote_count }`
  - `TagPills.svelte` updated: tags are now clickable upvote buttons with ▲ indicator and live vote count
  - `auth.resolve_user_id()` extracted as shared public helper (was duplicated in reviews.rs and users.rs)
  - Frontend `voteTag()` API function added

- **Event sponsors stored in DB**
  - `event_sponsors` table: `id`, `event_id`, `name`, `logo_url`, with unique constraint on `(event_id, LOWER(name))`
  - `EventSponsor` model in backend, fetched and included in event detail API response
  - `insert_event_sponsors()` added to crawler `db.py` for pipeline integration
  - Frontend `EventSponsorRef` type added to `EventDetail` interface
  - Note: sponsor scraping itself remains disabled in crawler pipeline (Playwright sync/async incompatibility)

- **Global location filter**
  - `locationStore` (`$lib/stores/location.svelte.ts`) — Svelte 5 runes-based reactive store with localStorage persistence
  - Auto-detect via Geolocation API + Nominatim reverse geocoding (city/state label)
  - Manual city/state/country text input with autocomplete dropdown (keyword match against known event locations)
  - `GET /api/events/locations` backend endpoint — `SELECT DISTINCT location` query, no client-side extraction needed
  - Frontend `getUniqueLocations()` calls the dedicated endpoint, module-level cache (per-session, no TTL)
  - Suggestions list: case-insensitive substring filter, max 8 results, shows top 8 on empty input for browsing
  - Nav header integration (desktop dropdown + mobile section) with MapPin/Locate icons
  - Events page wired to filter by location (case-insensitive substring match on `event.location`)
  - Outside-click-to-close overlay for desktop dropdown

- **Events page: date range filter**
  - Custom `DatePicker.svelte` component replacing native browser date inputs — black bg, monospace text, no border-radius, month/year navigation, today highlighted in accent, selected date inverted (white on black), click-outside-to-close
  - Two date pickers (From / To) in a second toolbar row, separated by `border-t`
  - Client-side filtering on `start_date` — events outside the range are excluded from results
  - Clear button appears when either date is set; results count updates to reflect filtered subset
  - All labels use `text-text` (white), not dim or grey, per design spec

- **Companies page: "Most Recent Event" sort**
  - New `latest_event_date` field in `CompanySummaryResponse` — correlated subquery: `MAX(e.start_date)` via `event_companies` join
  - Added to `CompanySummary` (FromRow) and `CompanySummaryResponse` structs in backend
  - Frontend sort dropdown now has 14 options: Name, Overall Rating, Events Hosted, Most Recent Event + 10 rating categories
  - Defaults to descending (most recent first) when selected

- **Compare page: inline search with entity selection**
  - Debounced search input (300ms) calls `/api/search` filtered by type (event or company)
  - Dropdown shows matching results with score badge and review count; click to add
  - Selected entities shown as removable chips; URL updates via `goto()` for shareable links
  - Events / Companies type toggle clears selection and filters search results
  - Max 4 entities (compare API limit); comparison grid renders at 2+
  - Replaces broken flow that redirected to `/search` with no way back
  - Dev-only mock data: 4 mock events + 4 mock companies, selectable via toggle, gated behind `esm-env` DEV (tree-shaken in prod)
- **About page** (`/about`) — editorial brutalist about page with mission statement, 3-step "How It Works" flow (Crawl → Enrich → Review), 2-column tech stack grid, data sources (MLH, Luma, CV, Hackiterate), 10 rating categories grid, open source links (GitHub repo)
- **API documentation page** (`/api`) — interactive API reference with overview cards (base URL, auth, format), quick-reference endpoint table, 7 expandable sections (Events, Companies, Users, Reviews, Search, Tags, Compare) with method color-coding, auth badges, request/response examples, architecture note with correlated subquery example

- **Events page: sort, search, and view toggle**
  - Client-side search bar filters events by name or location in real time
  - Sort dropdown with 13 options: Date, Name, Overall Rating + all 10 category ratings (Organization, Prizes, Mentorship, Judging, Venue, Food & Drinks, Swag, Networking, Communication, Vibes) with ascending/descending toggle
  - When sorting by a category, the rating column shows that category's average instead of overall rating
  - Display toggle: list view (default) or grid view (existing EventCard layout)
  - List view matches companies page layout: color-coded rating on left (green/yellow/red), event name + date + review count center, location right
  - `GET /api/events` now returns `category_ratings` (per-category averages) in `EventSummary`, batch-fetched in one query (no N+1)
  - Fetches all events in single request (per_page 500) for client-side filtering
  - Dev-only placeholder events via `$app/environment` `dev` flag (tree-shaken in production builds)

- **Companies page: list layout with sort & search**
  - Replaced 3-column card grid with flat list view (rating left, company name center, events hosted right)
  - Client-side search bar with clear button, filters by name or description
  - Sort dropdown matching events page: General (Name, Overall Rating, Events Hosted) + Category (all 10 rating categories) with ↑/↓ direction toggle
  - When sorting by a category, the rating column shows that category's average instead of overall rating
  - `GET /api/companies` now returns `avg_rating`, `review_count`, and `category_ratings` alongside `event_count`; new `?search=` query param for server-side ILIKE name filtering
  - Backend batch-fetches category averages for all listed companies in one query (no N+1)
  - Dev-only placeholder companies via `dev` flag (tree-shaken in production)

### Fixed

- **Clerk sign-in/sign-up dark theme** — Clerk components used default white card on black background. Added `appearance` prop to `ClerkProvider` with dark brutalist styling: black card bg, white text, `#111` inputs with `#333` borders, white primary button (inverted), `Space Mono` font, `Instrument Serif` italic header, 0px border-radius, 560px card width. Removed redundant "Sign In"/"Sign Up" headings. Centered via flex without constraining wrapper.
- **Globe memory leak / browser freeze on reverse scroll** — cobe WebGL render loop (60fps, 16K mapSamples) never paused when scrolled past the pinned section. Globe kept burning GPU at full resolution even at opacity 0 off-screen. Added `onLeave`/`onEnterBack` ScrollTrigger callbacks to toggle `visible` prop, which calls `globe.toggle(false)` to pause cobe's rAF loop when the section is scrolled past and resumes on scroll-back.
- **ScrollTrigger ghost layout on navigation** — pin spacer (~5000px) persisted during client-side navigation, pushing new page content below a blank gap. `beforeNavigate` now calls `ctx.revert()` before SvelteKit swaps pages.
- **Globe too dim** — increased `diffuse` (1.2→2), `mapBrightness` (2→4), `baseColor` (0.2→0.35), `glowColor` (0.1→0.15), raised marker brightness floors

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
  - Fixed oval globe: canvas `aspect-square` + cobe official resize pattern
  - `requestAnimationFrame` for cobe creation (child-before-parent mount timing)
  - Date-based marker brightness (closer = brighter)
  - Scroll-driven showcase: hero → morph → event cycling → exit
  - Globe zoom once at morph, not per-event pulse
  - Scroll morph: cobe renders at showcase size, hero is CSS scale-up — pure transform morph (scale/x/y), no layout changes, no cobe resize
  - Hero text `pointer-events-none` so globe is draggable through text overlay
  - Cobe official resize pattern — cached width via event listener, not per-frame `offsetWidth`
  - Dynamic `mapSamples`: 16K hero / 8K showcase (perf)
  - Globe visibility toggle: `globe.toggle(false/true)` pauses/resumes cobe when scrolled past
  - Marker sampling: cap at 200 markers for GPU performance (down from 1600+)
  - DPR capping: `Math.min(rawDpr, MAX_RENDER_PX / width)` limits GPU buffer without affecting display size
  - Removed ScrollTrigger `snap` — caused lag on reverse scroll
  - Globe `destroyed` guard prevents cobe rendering after navigation
  - `will-change: transform` on globe container for GPU compositor isolation
  - `data-sveltekit-preload-data="off"` on EventCards — prevents 404 flood on hover
  - `beforeNavigate` kills ScrollTrigger before SvelteKit client-side nav
  - Globe surface tuned (`mapBrightness: 2`, `baseColor: [0.2,0.2,0.2]`) so hackathon markers stand out
  - Marker dots enlarged to `size: 0.1` with per-marker color based on `start_date`
  - Larger hero globe (90% viewport) positioned at top 45% for more prominent hero presence
  - Hero layout: "Every hackathon," on one line, search bar beside subtext (larger input)
  - Events page: back button, removed Archive label
  - Pull quote: removed "What we believe", `#111` bg
  - Docker frontend runtime: `node` → `bun`
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
