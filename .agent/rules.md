---
description: Project rules for RateMyHackathons
---

# Agent Rules

## Documentation Requirements
After every significant run or set of changes, **always** update:
1. `TODO.md` — mark completed items, add new ones
2. `README.md` — if new features, services, or setup steps were added
3. `CHANGELOG.md` — log what changed and when
4. Service-specific READMEs (`backend/README.md`, `services/crawler/README.md`, `services/analytics/README.md`) — if those services were modified

## TODO Reference
Always check `TODO.md` at the start of work to understand current priorities and avoid duplicate effort.

## Environment
- Never overwrite `.env` files — they contain user secrets
- Never force push
- Use `uv` for Python package management in `services/crawler/`
- Use `bun` for JS/TS in `services/analytics/dashboard/`

## Crawler
- All spiders use Scrapling (`scrapling.fetchers`)
- Use `StealthyFetcher` for JS-rendered sites (SPAs)
- Use `Fetcher` for static HTML sites
- LLM sponsor extraction uses OpenRouter with free model rotation (paid fallback)
- Test spider changes with sandbox scripts in `/tmp/` before modifying production code
