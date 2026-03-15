# RateMyHackathons — TODO

> Last updated: 2026-03-13 (API recon complete)

## 🔴 In Progress

- [ ] Fix LLM sponsor extraction — add paid model fallback for reliability
- [x] Investigate AIDA MCP for discovering event data endpoints (CV, Luma, etc.)
  - See `services/crawler/cv/API_RECON.md` for full report
- [x] Build CV (Cerebral Valley) spider — **PUBLIC API cracked: `GET /v1/public/event/pull?featured=true`** (no auth!)
  - ✅ Dry run: **9,258 events** extracted (featured + approved)
- [x] Build Luma spider — **open API at `api.lu.ma/discover/get-paginated-events`** (no auth, JSON, cursor-paginated)
  - ✅ Dry run: **920 events** extracted (20 pages)

## 🟡 Planned

- [ ] Store scraped sponsors in DB (new `event_sponsors` table)
- [ ] Deduplicate sponsor names across events (normalize casing, abbreviations)
- [ ] Add `--skip-sponsors` flag for bulk crawl runs
- [ ] Rate-limit sponsor scraping (avoid hammering event sites)
- [ ] Dashboard: sponsor analytics panel (top sponsors, sponsor frequency)
- [ ] Integration tests for spider → DB pipeline

## 🟢 Completed

- [x] MLH spider — 194 events, correct name/location parsing
- [x] Hackiterate spider — 6 events via StealthyFetcher (Playwright)
- [x] Sponsor scraper — 4-strategy extraction (CSS class/id, headings, src path, LLM fallback)
- [x] LLM module — dynamic free model discovery from OpenRouter `/api/v1/models`
- [x] Provider-priority rotation (Google first)
- [x] Analytics Rust API + SvelteKit dashboard
- [x] Backend API (Actix-web, PostgreSQL)
- [x] Sub-READMEs with UML diagrams for backend, services, crawler, analytics
- [x] Root README with doc links, getting started guide
- [x] CHANGELOG

## 📝 Notes

- Free OpenRouter models: 20 req/min, 1000/day (with $10+ credits)
- Use paid models as fallback when free tier is congested
- Most hackathon sites are SPAs — use StealthyFetcher for JS rendering
- Sponsor extraction: CSS strategies work for ~70% of sites, LLM handles the rest
