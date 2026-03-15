# RateMyHackathons Codebase Review

Date: 2026-03-15

Scope reviewed:
- frontend
- backend
- services/crawler
- services/analytics
- README/TODO/docs drift

Notes:
- This review reflects the current working tree, which already contains in-progress Phase 2 Clerk/auth work.
- Crawler validation was run live with `services/crawler/.venv/bin/python dry_run.py`.

## Critical Issues

- Search is wired to the wrong API contract. The backend returns compact search DTOs with `name` and `rank`, but the frontend types and UI assume full `EventSummary`, `Company`, and `User` objects. Event hits are passed into `EventCard`, which expects `companies`, and user hits render `user.username`, which the backend never sends.
  Refs: [backend/src/routes/search.rs:17](/Users/xiao/ratemyhackathons/backend/src/routes/search.rs#L17), [backend/src/routes/search.rs:25](/Users/xiao/ratemyhackathons/backend/src/routes/search.rs#L25), [frontend/src/lib/types.ts:119](/Users/xiao/ratemyhackathons/frontend/src/lib/types.ts#L119), [frontend/src/routes/search/+page.svelte:62](/Users/xiao/ratemyhackathons/frontend/src/routes/search/+page.svelte#L62), [frontend/src/routes/search/+page.svelte:93](/Users/xiao/ratemyhackathons/frontend/src/routes/search/+page.svelte#L93), [frontend/src/lib/components/EventCard.svelte:74](/Users/xiao/ratemyhackathons/frontend/src/lib/components/EventCard.svelte#L74)

- Search failures fall into a blank UI state. The loader returns `results: null` on fetch failure, but the page only renders for `data.results` or empty-query mode, so `?q=...` plus API failure yields no fallback message.
  Refs: [frontend/src/routes/search/+page.ts:7](/Users/xiao/ratemyhackathons/frontend/src/routes/search/+page.ts#L7), [frontend/src/routes/search/+page.svelte:42](/Users/xiao/ratemyhackathons/frontend/src/routes/search/+page.svelte#L42), [frontend/src/routes/search/+page.svelte:104](/Users/xiao/ratemyhackathons/frontend/src/routes/search/+page.svelte#L104)

- Fresh installs following the root README are incomplete. The setup instructions stop before `20260314_rmp_ratings.sql`, but the current backend depends on `review_ratings`, `tags`, `review_tags`, nullable `event_id`, `company_id`, and `would_return`; the README's `POST /api/reviews` schema is still pre-Phase-1.
  Refs: [README.md:224](/Users/xiao/ratemyhackathons/README.md#L224), [backend/migrations/20260314_rmp_ratings.sql:1](/Users/xiao/ratemyhackathons/backend/migrations/20260314_rmp_ratings.sql#L1), [README.md:451](/Users/xiao/ratemyhackathons/README.md#L451), [backend/src/routes/users.rs:168](/Users/xiao/ratemyhackathons/backend/src/routes/users.rs#L168)

- Analytics is not coherent with the new review model. `recent_reviews` maps nullable `reviews.title` into `String` and inner-joins only `event_id`, so untitled reviews can break the endpoint and company reviews never appear in the dashboard.
  Refs: [services/analytics/src/db.rs:169](/Users/xiao/ratemyhackathons/services/analytics/src/db.rs#L169), [services/analytics/src/db.rs:181](/Users/xiao/ratemyhackathons/services/analytics/src/db.rs#L181), [backend/migrations/20260313_initial_schema.sql:53](/Users/xiao/ratemyhackathons/backend/migrations/20260313_initial_schema.sql#L53), [backend/migrations/20260314_rmp_ratings.sql:38](/Users/xiao/ratemyhackathons/backend/migrations/20260314_rmp_ratings.sql#L38)

- A clean Postgres database can fail at the crawl-registry migration because `gen_random_uuid()` is used without enabling `pgcrypto`.
  Ref: [backend/migrations/20260313_crawl_registry.sql:3](/Users/xiao/ratemyhackathons/backend/migrations/20260313_crawl_registry.sql#L3)

## Warnings

- The landing page no longer matches the documented 7-section build. The implemented page has 6 sections plus a marquee divider, omits Companies and Recent Reviews, and the "How it works" section is not pinned/content-swapped as documented.
  Refs: [frontend/src/routes/+page.svelte:40](/Users/xiao/ratemyhackathons/frontend/src/routes/+page.svelte#L40), [frontend/src/routes/+page.svelte:152](/Users/xiao/ratemyhackathons/frontend/src/routes/+page.svelte#L152), [README.md:151](/Users/xiao/ratemyhackathons/README.md#L151), [docs/FRONTEND_PLAN.md:31](/Users/xiao/ratemyhackathons/docs/FRONTEND_PLAN.md#L31)

- Demo fallback is inconsistent on the landing page: cards fall back to `demoEvents`, but the globe gets an empty marker array when `/api/events/globe` fails.
  Refs: [frontend/src/routes/+page.ts:6](/Users/xiao/ratemyhackathons/frontend/src/routes/+page.ts#L6), [frontend/src/routes/+page.svelte:30](/Users/xiao/ratemyhackathons/frontend/src/routes/+page.svelte#L30), [frontend/src/routes/+page.svelte:43](/Users/xiao/ratemyhackathons/frontend/src/routes/+page.svelte#L43)

- The frontend API client still contains stale write/read contracts. `createReview` posts to `/users/{id}/reviews`, `voteReview` sends `{ vote }`, and `getReview` and `listComments` types do not match backend responses. Current pages do not use these helpers yet, but any review-writing UI built on them will fail.
  Refs: [frontend/src/lib/api.ts:58](/Users/xiao/ratemyhackathons/frontend/src/lib/api.ts#L58), [frontend/src/lib/api.ts:62](/Users/xiao/ratemyhackathons/frontend/src/lib/api.ts#L62), [frontend/src/lib/api.ts:69](/Users/xiao/ratemyhackathons/frontend/src/lib/api.ts#L69), [frontend/src/lib/api.ts:76](/Users/xiao/ratemyhackathons/frontend/src/lib/api.ts#L76), [backend/src/routes/users.rs:168](/Users/xiao/ratemyhackathons/backend/src/routes/users.rs#L168), [backend/src/routes/reviews.rs:148](/Users/xiao/ratemyhackathons/backend/src/routes/reviews.rs#L148), [backend/src/routes/reviews.rs:190](/Users/xiao/ratemyhackathons/backend/src/routes/reviews.rs#L190), [backend/src/routes/reviews.rs:247](/Users/xiao/ratemyhackathons/backend/src/routes/reviews.rs#L247)

- Sponsor extraction exists but is disabled in the real crawl pipeline, so README and TODO overstate what is actually running.
  Refs: [services/crawler/main.py:137](/Users/xiao/ratemyhackathons/services/crawler/main.py#L137), [services/crawler/sponsors.py:116](/Users/xiao/ratemyhackathons/services/crawler/sponsors.py#L116), [services/crawler/README.md:148](/Users/xiao/ratemyhackathons/services/crawler/README.md#L148), [TODO.md:66](/Users/xiao/ratemyhackathons/TODO.md#L66)

- Crawler correctness still has edge-case risk. Fuzzy duplicate detection is name-only within a date window, geocode caching is only in-memory, and CV URLs are not trimmed before scheme detection, which can produce malformed prefixed URLs from whitespacey upstream values.
  Refs: [services/crawler/main.py:61](/Users/xiao/ratemyhackathons/services/crawler/main.py#L61), [services/crawler/dedup.py:100](/Users/xiao/ratemyhackathons/services/crawler/dedup.py#L100), [services/crawler/geocode.py:72](/Users/xiao/ratemyhackathons/services/crawler/geocode.py#L72), [services/crawler/spiders/cerebralvalley.py:182](/Users/xiao/ratemyhackathons/services/crawler/spiders/cerebralvalley.py#L182)

- The crawl and review schema still lacks some defensive constraints and index coverage for actual query paths: nullable `source_hash`, no FK from `crawl_sources` to `scrape_sources`, single-column review indexes where handlers sort by `created_at`, and no lat/lng range checks.
  Refs: [backend/migrations/20260313_crawl_registry.sql:14](/Users/xiao/ratemyhackathons/backend/migrations/20260313_crawl_registry.sql#L14), [backend/migrations/20260313_initial_schema.sql:59](/Users/xiao/ratemyhackathons/backend/migrations/20260313_initial_schema.sql#L59), [backend/migrations/20260313_initial_schema.sql:75](/Users/xiao/ratemyhackathons/backend/migrations/20260313_initial_schema.sql#L75), [backend/migrations/20260314_rmp_ratings.sql:68](/Users/xiao/ratemyhackathons/backend/migrations/20260314_rmp_ratings.sql#L68), [backend/migrations/20260314_event_geocoding.sql:5](/Users/xiao/ratemyhackathons/backend/migrations/20260314_event_geocoding.sql#L5)

- Services documentation is stale: it still marks Cerebral Valley and Lu.ma as future work even though both scrapers are wired in `main.py`.
  Refs: [services/README.md:28](/Users/xiao/ratemyhackathons/services/README.md#L28), [services/README.md:29](/Users/xiao/ratemyhackathons/services/README.md#L29), [services/crawler/main.py:31](/Users/xiao/ratemyhackathons/services/crawler/main.py#L31), [services/crawler/main.py:32](/Users/xiao/ratemyhackathons/services/crawler/main.py#L32)

## Suggestions

- Unify the search contract first. Either make the frontend consume the compact search DTOs it actually gets, or have `/api/search` return card-ready event/company/user payloads.

- Update docs in one sweep: root `README`, `services/README`, `docs/FRONTEND_PLAN.md`, and `TODO.md` should reflect the current migration set, Clerk env requirements, actual landing-page scope, and the fact that sponsor extraction is not active.

- Normalize the crawl model around a real `scrape_source_id` FK, make `source_hash` non-null, add composite review indexes, and add coordinate/category constraints at the DB layer instead of relying on handler code.

- Add a small integration test matrix for API contracts that matter to the frontend: `/api/search`, `/api/reviews/{id}`, and the review write flows.

## Build Health

- `frontend`: `bun run check` passed with `0` errors and `5` warnings, all in [frontend/src/routes/+layout.svelte:33](/Users/xiao/ratemyhackathons/frontend/src/routes/+layout.svelte#L33), [frontend/src/routes/+layout.svelte:11](/Users/xiao/ratemyhackathons/frontend/src/routes/+layout.svelte#L11), and [frontend/src/routes/search/+page.svelte:8](/Users/xiao/ratemyhackathons/frontend/src/routes/search/+page.svelte#L8)

- `backend`: `cargo check` passed with `0` errors and `4` warnings in [backend/src/auth.rs:24](/Users/xiao/ratemyhackathons/backend/src/auth.rs#L24), [backend/src/auth.rs:126](/Users/xiao/ratemyhackathons/backend/src/auth.rs#L126), [backend/src/errors.rs:10](/Users/xiao/ratemyhackathons/backend/src/errors.rs#L10), and [backend/src/models/event.rs:39](/Users/xiao/ratemyhackathons/backend/src/models/event.rs#L39)

- `crawler`: plain `python dry_run.py` failed under the system interpreter because `scrapling` was missing; `./.venv/bin/python dry_run.py` succeeded live on March 14, 2026 and returned non-empty data from all four spiders: CV `975`, Lu.ma `15`, MLH `233`, Hackiterate `6`, `1204` after cross-source dedup

- `analytics`: `cargo check` did not complete in the sandbox because Cargo needed to download crates from `static.crates.io` and DNS resolution failed during dependency fetch

## Security and Quality Notes

- I did not find obvious SQL injection vectors; DB access is parameterized throughout.

- I did not find `@html` or `innerHTML` usage in the frontend.

- `ammonia` is applied consistently on backend write handlers, but crawler DB writes bypass that sanitation path.
