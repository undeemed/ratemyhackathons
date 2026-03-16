# RateMyHackathons — TODO

> Last updated: 2026-03-15

## Planned

(none — all current items implemented)

## Completed

- [x] **Homepage autocomplete search bar** — `SearchAutocomplete.svelte` component with 300ms debounced search via existing `/api/search` endpoint, live dropdown with score badges and review counts, keyboard navigation (arrow keys + escape), click-to-navigate to detail pages, Enter falls through to `/search` results page. Replaces static form in hero section.
- [x] **Tag voting** — `tag_votes` table (user_id + tag_id unique), `POST /api/tags/{id}/vote` toggle endpoint (insert/delete), `TagPills.svelte` updated with upvote buttons and live vote counts. Backend `resolve_user_id` moved to `auth.rs` as shared helper.
- [x] **Store scraped sponsors in DB** — `event_sponsors` table with unique constraint on `(event_id, LOWER(name))`, `EventSponsor` model in backend, sponsors fetched and returned in event detail response, `insert_event_sponsors()` added to crawler `db.py`. Sponsor scraping itself remains disabled (Playwright sync/async incompatibility).
- [x] **Deploy analytics dashboard** — analytics service in `docker-compose.yml` (Dockerfile.analytics, binds `:8081`)
- [x] **Fix LLM sponsor extraction** — paid model fallback (`google/gemini-2.0-flash-001`) in `llm.py` when all free OpenRouter models fail

- [x] **Location filter** — global location filter in the nav header (auto-detect via Geolocation API + manual city/state/country input), autocomplete dropdown with keyword match via dedicated `GET /api/events/locations` endpoint (`SELECT DISTINCT location`), `locationStore` with localStorage persistence, filters events page by substring match on `event.location`. Frontend caches location list per session (module-level, no TTL).

- [x] **Date range filter** — custom `DatePicker.svelte` component (black bg, monospace, no border-radius, month/year nav, click-outside-to-close) replacing native browser date inputs, client-side filtering on `start_date`, clear button, white labels (`text-text`)
- [x] **Companies page: "Most Recent Event" sort** — `latest_event_date` correlated subquery (`MAX(e.start_date)` via `event_companies`) in backend, 14 sort options in frontend dropdown (4 general + 10 categories)
- [x] **Compare page: inline search with entity selection** — debounced search dropdown filtered by event/company type, click-to-add chips, `goto()` URL sync for shareable links, max 4 entities, dev-only mock data (4 events + 4 companies) gated behind `esm-env` DEV
- [x] **About page (`/about`)** — mission, how-it-works, tech stack (2-col grid), data sources, rating categories, open source links
- [x] **API docs page (`/api`)** — interactive endpoint reference with expandable sections, method color-coding, auth badges, request/response examples
- [x] **Companies page: list layout with sort & search** — flat list view (rating/name/events hosted), client-side search, sort dropdown with 14 options (name/rating/events/latest event + 10 categories), category sort swaps rating column to show category avg, backend returns avg_rating + review_count + category_ratings + latest_event_date + search param
- [x] **Fix: Clerk sign-in/sign-up dark theme** — dark brutalist appearance on ClerkProvider, removed redundant headings, centered + enlarged card
- [x] **Events page: sort, search, and view toggle** — client-side search/sort/display-mode toolbar, sort dropdown with 13 options (date/name/rating + 10 categories), category sort swaps rating column to show category avg, backend returns `category_ratings` in EventSummary (batch-fetched), list default
- [x] **Fix: Globe memory leak / browser freeze on reverse scroll** — cobe render loop never paused when off-screen; added `onLeave`/`onEnterBack` to toggle `visible` prop
- [x] **Fix: ScrollTrigger ghost layout on navigation** — `beforeNavigate` kills pin spacer before SvelteKit swaps pages
- [x] **Fix: Globe too dim** — bumped diffuse, mapBrightness, baseColor, glowColor, marker brightness floors
- [x] **Internal Admin Dashboard** — multi-page analytics dashboard
  - [x] Sidebar navigation (Overview, Events, Crawl Sources, Reviews)
  - [x] Overview: entity counts, crawl stats, live SSE feed, trending, ratings, recent reviews
  - [x] Events browser: paginated table with search/filter, color-coded ratings, company tags
  - [x] Crawl Sources: source health table, by-type breakdown, 30d history bar chart
  - [x] Reviews: rating distribution with %, trending events, recent reviews table
  - [x] Main API proxy (`/main-api` → `:8080`) for event/company/user data
- [x] **Hero UX improvements**
  - [x] "rated." gradient text (red → yellow → green matching score colors)
  - [x] Search mode toggle: Hackathons / Companies with dynamic placeholder
  - [x] WIP popup: shows once only (localStorage), "Enter anyways" button
  - [x] Recent events: flat 3-column grid (removed oversized `row-span-2` magazine layout)
- [x] **Globe & Landing Page Overhaul**
  - [x] Fixed oval globe: canvas `aspect-square` + cobe official resize pattern
  - [x] `requestAnimationFrame` for cobe creation (child-before-parent mount timing)
  - [x] Date-based marker brightness (closer = brighter)
  - [x] Scroll-driven showcase: hero → morph → event cycling → exit
  - [x] Globe zoom once at morph, not per-event pulse
  - [x] Scroll perf fix: transform-only GSAP morph (`x`/`y`/`scale`) — no forced reflows
  - [x] Cobe official resize pattern — cached width via event listener, not per-frame `offsetWidth`
  - [x] Dynamic `mapSamples`: 20K hero / 12K showcase
  - [x] Removed ScrollTrigger snap — caused lag on reverse scroll
  - [x] Globe destroyed guard + `will-change: transform` compositor isolation
  - [x] `beforeNavigate` kills ScrollTrigger before SvelteKit nav
  - [x] Hero layout: single-line heading, search bar beside subtext
  - [x] Events page: back button, Docker `node` → `bun`
- [x] **Phase 4: Advanced Features**
  - [x] Compare tool (`/compare` page with side-by-side category comparison)
  - [x] Compare link in nav (desktop + mobile)
  - [x] Shareable compare URLs
- [x] **Phase 3: Frontend Rebuild** — RMP-style color-coded scores
  - [x] Updated `types.ts`: CategoryAvg, TagCount, RatingDistribution, CompanyDetail, CompareEntity, CreateReviewPayload, score helpers
  - [x] Updated `api.ts`: getCompany returns CompanyDetail, compare(), listTags(), createTag()
  - [x] Score color theme vars: `--color-score-green/yellow/red`
  - [x] New components: ScoreBadge, CategoryGrid, RatingDistribution, TagPills, RatingSlider
  - [x] Updated ReviewCard with category breakdowns + would-return badge
  - [x] RMP-style search results (score badge + name + would-return %)
  - [x] Company detail page (giant score, category grid, top tags, rating distribution, events list, reviews)
  - [x] Event detail page (same structure with image, dates, companies)
  - [x] Rate form for events (`/events/{id}/rate`) with auth gate, 10 category sliders, tags, review text
  - [x] Rate form for companies (`/companies/{id}/rate`) — same structure
- [x] **Phase 2: Clerk Auth** — svelte-clerk frontend + Rust JWT middleware
- [x] **Phase 1: Schema + API** — Multi-dimensional ratings
- [x] **Frontend v2** — Editorial brutalist B&W redesign
- [x] **Frontend v1** — SvelteKit app with cobe globe hero + GSAP scroll animations
- [x] MLH spider — 194 events
- [x] Hackiterate spider — 6 events via StealthyFetcher (Playwright)
- [x] CV spider — 9,258 events via public API
- [x] Luma spider — 920 events via discover API
- [x] Sponsor scraper — 4-strategy extraction
- [x] LLM module — dynamic free model discovery from OpenRouter
- [x] Analytics Rust API + SvelteKit dashboard
- [x] Backend API (Actix-web, PostgreSQL)

## Notes

- Free OpenRouter models: 20 req/min, 1000/day (with $10+ credits)
- Frontend design: B&W editorial brutalist, Instrument Serif (italic) + Space Mono, no border-radius
- Score colors: green (#4caf50) ≥4.0, yellow (#ffc107) ≥3.0, red (#ef5350) <3.0
- Rate form requires: all 10 categories rated + 350-5000 char review body
- Auth: Clerk (signed-in required for reviews), dev mode falls back to body user_id
- Admin dashboard: SvelteKit on :5174, reads from analytics API (:8081) + main API (:8080)
- **Location autocomplete cache**: `getUniqueLocations()` in `api.ts` calls `GET /api/events/locations` (dedicated `SELECT DISTINCT` query), caches result in module-level `_locationsCache` with no TTL. Cache lives for the browser tab session. Lightweight — no Redis or heavy infra needed.
