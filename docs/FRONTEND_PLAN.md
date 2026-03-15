# Frontend Implementation Plan

## Context
RateMyHackathons needs a public frontend. Backend API (Rust/Actix `:8080`) and crawler (Python) exist. This adds a SvelteKit app with a WebGL globe hero, GSAP scroll animations, and full CRUD pages.

## Tech Stack
- **SvelteKit** (Svelte 5 runes) + **bun** + **Tailwind v4** + **TypeScript**
- **cobe** (5kB WebGL globe) + **GSAP** + ScrollTrigger + **lucide-svelte**
- Dark theme matching analytics dashboard (`#0a0a0f` base, indigo accents)
- **Always use context7 MCP** (`mcp__plugin_context7_context7`) to look up docs for SvelteKit, GSAP, cobe, Tailwind, and any other library before writing code

## Changes

### 1. DB Migration: `backend/migrations/20260314_event_geocoding.sql`
Add `latitude DOUBLE PRECISION` and `longitude DOUBLE PRECISION` to `events` table + spatial index.

### 2. Backend Model/Route Changes
- `backend/src/models/event.rs` — add lat/lng fields to all structs
- `backend/src/routes/events.rs` — add lat/lng to queries + new `GET /api/events/globe` returning `[{id, name, lat, lng, start_date}]`

### 3. Crawler Geocoding: `services/crawler/geocode.py`
- Nominatim geocoding via `geopy` with local cache dict
- Integrate into `services/crawler/main.py` `process_events()` — geocode after insert
- Add `geopy>=2.4` to `services/crawler/pyproject.toml`

### 4. Frontend App: `frontend/`
Scaffold with `bun create svelte@latest`, add `gsap cobe lucide-svelte @tailwindcss/vite tailwindcss`.

Vite proxies `/api` → `http://127.0.0.1:8080`. Dev server on `:5173`.

### 5. Landing Page Storyboard (7 sections)
1. **Hero** — Full-viewport cobe globe with pulsing dots at hackathon locations. Headline: "Every Hackathon, Rated." Search bar. Globe parallax-tilts on mouse. On scroll: globe shrinks left, text slides right.
2. **Stats Counter** — 4 animated counters (events, cities, companies, sources) counting up via GSAP.
3. **Trending Events** — Card grid with staggered GSAP reveal. EventCards: image, name, location, date, rating.
4. **How It Works** — Pinned 3-step section (Discover → Experience → Rate) with content swapping on scroll.
5. **Companies** — Logo cloud from `/api/companies`. Stagger fade-in.
6. **Recent Reviews** — 3 featured ReviewCards with slide-in animations.
7. **CTA + Footer** — Gradient banner + footer links.

### 6. Inner Pages
- `/events` — Paginated grid, search/filter, sort (newest/top-rated)
- `/events/[id]` — Event detail, reviews, threaded comments, write review form
- `/companies` + `/companies/[id]` — Directory + detail with sponsored events
- `/users/[id]` — Profile with review history
- `/search` — Tabbed results (events/companies/users) via `/api/search`

### 7. Core Components
- `Globe.svelte` — cobe init/destroy, markers prop, auto-rotate, mouse parallax
- `lib/api.ts` — Typed fetch wrapper for all endpoints
- `lib/types.ts` — TS interfaces matching backend DTOs
- `lib/animations/gsap.ts` — ScrollTrigger setup + Svelte actions (`use:fadeIn`, `use:slideUp`, `use:staggerChildren`, `use:countUp`)

### 8. Doc Updates
- **CLAUDE.md** — Add frontend commands, context7 MCP rule, architecture
- **README.md** — Update tech stack table, project structure, getting started
- **TODO.md** — Mark frontend in-progress, add new items
- **CHANGELOG.md** — Log frontend addition
- Export this plan as `docs/FRONTEND_PLAN.md`

## Implementation Order
1. DB migration + backend changes (lat/lng + globe endpoint)
2. Crawler geocoding
3. Frontend scaffold + theme + layout
4. API client + types
5. GSAP animations + Globe component
6. Landing page storyboard
7. Inner pages (events → companies → users → search)
8. Doc updates (CLAUDE.md, README, TODO, CHANGELOG)

## Verification
- `cd backend && cargo test` — existing tests pass
- `curl localhost:8080/api/events/globe` — returns coords
- `cd frontend && bun dev` — globe renders, scroll animations work
- Navigate all pages, verify data loads from API
- `bun run check` — no TS errors
