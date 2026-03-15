# RateMyHackathons — TODO

> Last updated: 2026-03-14 (frontend implemented)

## 🔴 In Progress

- [ ] Fix LLM sponsor extraction — add paid model fallback for reliability

## 🟡 Planned

- [ ] Store scraped sponsors in DB (new `event_sponsors` table)
- [ ] Deduplicate sponsor names across events (normalize casing, abbreviations)
- [ ] Add `--skip-sponsors` flag for bulk crawl runs
- [ ] Rate-limit sponsor scraping (avoid hammering event sites)
- [ ] Dashboard: sponsor analytics panel (top sponsors, sponsor frequency)
- [ ] Integration tests for spider → DB pipeline
- [ ] Frontend: auth (login/signup)
- [ ] Frontend: write review form on event detail page
- [ ] Frontend: responsive polish + accessibility audit

## 🟢 Completed

- [x] **Frontend** — SvelteKit app with cobe globe hero + GSAP scroll animations
  - [x] DB migration: add lat/lng to events table
  - [x] Backend: add lat/lng to models/routes + `GET /api/events/globe`
  - [x] Crawler: geocoding module (Nominatim via geopy)
  - [x] Frontend scaffold (SvelteKit + Tailwind v4 + bun)
  - [x] Landing page storyboard (7 sections with GSAP ScrollTrigger)
  - [x] Inner pages (events, companies, users, search)
- [x] MLH spider — 194 events, correct name/location parsing
- [x] Hackiterate spider — 6 events via StealthyFetcher (Playwright)
- [x] CV spider — 9,258 events via public API
- [x] Luma spider — 920 events via discover API
- [x] Sponsor scraper — 4-strategy extraction
- [x] LLM module — dynamic free model discovery from OpenRouter
- [x] Analytics Rust API + SvelteKit dashboard
- [x] Backend API (Actix-web, PostgreSQL)
- [x] API recon for CV, Luma, MLH, Hackiterate
- [x] Root README with doc links, getting started guide
- [x] CLAUDE.md with context7 MCP requirement

## 📝 Notes

- Free OpenRouter models: 20 req/min, 1000/day (with $10+ credits)
- Use paid models as fallback when free tier is congested
- Most hackathon sites are SPAs — use StealthyFetcher for JS rendering
- Sponsor extraction: CSS strategies work for ~70% of sites, LLM handles the rest
- Frontend plan: `docs/FRONTEND_PLAN.md`
