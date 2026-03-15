# RateMyHackathons — TODO

> Last updated: 2026-03-15

## Planned

- [ ] **Performance: Globe/homepage GPU pressure** — hero globe can reach 1440px → 2880x2880 render target (~32MB color buffers). Add `LIMIT` or sampling to `/events/globe` endpoint, cap globe render dimensions
- [ ] **Performance: Hover detection O(n)** — `findNearestMarker()` scans all markers on every pointer move with trig math. Add spatial index or throttle
- [ ] **Bug: SectionNav kills all ScrollTriggers globally** — `SectionNav.svelte:48` should scope cleanup to its own triggers, not nuke everything
- [ ] Homepage autocomplete search bar (SearchAutocomplete component — live dropdown)
- [ ] Tag voting (upvote existing tags on reviews)
- [ ] Fix LLM sponsor extraction — add paid model fallback for reliability
- [ ] Store scraped sponsors in DB (new `event_sponsors` table)
- [ ] Frontend: responsive polish + accessibility audit
- [ ] Frontend: loading skeletons for API-dependent sections
- [ ] Deploy analytics dashboard alongside main app (Docker service)

## Completed

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
