# Crawler

Python scraping service that polls hackathon sources, deduplicates across platforms, enriches with host/organizer data, and auto-detects sponsoring companies.

## Architecture

```mermaid
classDiagram
    class Main {
        +cli()
        +async_main(args)
        +crawl_once(pool, dry_run)
        +daemon_loop(pool, interval)
        +process_events(pool, events, source_type)
    }
    class Db {
        +get_pool(url) Pool
        +get_enabled_sources(pool)
        +should_poll(pool, id, interval)
        +mark_polled(pool, id)
        +hash_exists(pool, hash)
        +insert_event(pool, event)
        +insert_crawl_source(pool, ...)
        +link_company_to_event(pool, ...)
        +get_all_companies(pool)
        +find_events_by_name_window(pool, ...)
    }
    class Dedup {
        +make_source_hash(type, url) str
        +normalize_event_url(url) str
        +normalize_name(name) str
        +levenshtein_ratio(s1, s2) float
        +is_fuzzy_duplicate(name, existing) str?
        +deduplicate_cross_source(events) list
    }
    class Proxy {
        +get_proxy_rotator() ProxyRotator?
        +get_single_proxy() str?
    }
    class Company {
        +detect_companies(text, known) list
    }
    class MLHSpider {
        +parse_mlh_date(text, year)
        +scrape_mlh(url, proxy) list
    }
    class HackiterateSpider {
        +parse_hackiterate_date(text)
        +scrape_hackiterate(url, proxy) list
    }
    class CVSpider {
        +scrape_cerebralvalley(url, proxy) list
        +_fetch_event_hosts(slug, opener) list
    }
    class LumaSpider {
        +scrape_luma(url, proxy) list
        +_is_hackathon_event(name, desc) bool
    }

    Main --> Db
    Main --> Dedup
    Main --> Proxy
    Main --> Company
    Main --> MLHSpider
    Main --> HackiterateSpider
    Main --> CVSpider
    Main --> LumaSpider
```

## Crawl Pipeline

```mermaid
flowchart LR
    A[Fetch events] --> X[Cross-source dedup]
    X --> B{Hash exists?}
    B -->|Yes| C[Skip]
    B -->|No| D{Fuzzy name match?}
    D -->|>0.85| E[Link to existing event]
    D -->|No match| F[Insert new event]
    F --> G[Detect companies]
    G --> H[Link companies]
```

## Cross-Source Deduplication

67% of CV events link to lu.ma/luma.com — same events appear in both spiders. The dedup layer:

1. **URL normalization** — strips UTM/tracking params, normalizes `luma.com` → `lu.ma`
2. **Cross-source grouping** — events with same canonical URL get grouped
3. **Smart merging** — keeps richest record (Luma priority), merges hosts from all sources

```
Before: 9,273 events (CV: 9,258 + Luma: 15)
After:  8,976 unique (187 duplicates merged)
```

## Spider Architecture

```mermaid
flowchart TB
    subgraph CVSpider["Cerebral Valley Spider"]
        C1["GET /v1/public/event/pull"] --> C2["Pull featured + approved"]
        C2 --> C3["Deduplicate by event ID"]
        C3 --> C4["GET /v1/event/{slug} for hosts"]
        C4 --> C5["Yield event + hosts"]
    end

    subgraph LumaSpider["Luma Spider"]
        L1["GET /discover/get-paginated-events"] --> L0["Multi-city geo sweep (15 cities)"]
        L0 --> L2["Cursor pagination per city"]
        L2 --> L3["Keyword filter (hackathon, buildathon...)"]
        L3 --> L4["Extract hosts, tickets, guests"]
        L4 --> L5["Yield enriched event dict"]
        L2 -->|has_more| L2
    end

    subgraph MLHSpider["MLH Spider"]
        M1["Fetch mlh.com/seasons/YYYY/events"] --> M2["CSS: a with utm_source"]
        M2 --> M3["Parse name, date, location"]
        M3 --> M7["Yield event dict"]
    end

    subgraph HackiterateSpider["Hackiterate Spider"]
        H1["Fetch hackiterate.com/directory"] --> H2["Playwright (JS rendering)"]
        H2 --> H3["Parse name, date, status"]
        H3 --> H7["Yield event dict"]
    end

    C5 & L5 & M7 & H7 --> Dedup["Cross-Source Dedup"]
    Dedup --> Pipeline["DB Insert Pipeline"]
```

## Enriched Data Per Source

| Field | Luma | CV | MLH | Hackiterate |
|---|---|---|---|---|
| Name | ✅ | ✅ | ✅ | ✅ |
| Dates | ✅ | ✅ | ✅ | ✅ start only |
| Location | ✅ | ✅ | ✅ | ✅ |
| Description | ✅ | ✅ | ❌ | ❌ |
| Image | ✅ | ✅ | ❌ | ❌ |
| **Hosts/Organizers** | ✅ (name, twitter, linkedin, website) | ✅ (name, isOrg, twitter, github) | ❌ | ❌ |
| **Guest count** | ✅ | ❌ | ❌ | ❌ |
| **Ticket info** | ✅ (free/paid, spots, sold out) | ❌ | ❌ | ❌ |
| **Timezone** | ✅ | ❌ | ❌ | ❌ |
| Event type | ❌ | ✅ (HACKATHON) | ❌ | ❌ |

### Sponsor Extraction

Each event page is visited to extract sponsor/partner names using 4 strategies:

| Strategy | Method | Coverage |
|---|---|---|
| CSS class/id | `[class*='sponsor']`, `[id*='partner']` → img alt text | ~40% |
| Heading detection | `<h2>Sponsors</h2>` → parent container imgs | ~20% |
| Src path matching | `<img src="/sponsors/rbc.svg" alt="RBC">` | ~10% |
| **LLM fallback** | Page text → OpenRouter (free models + paid Gemini Flash) | ~30% |

## Usage

```bash
# Setup
uv venv && uv pip install -r requirements.txt
cp .env.example .env   # set DATABASE_URL, PROXY_URL

# Dry-run (no DB needed)
python dry_run.py              # all sources with cross-source dedup
python dry_run.py cv           # Cerebral Valley only
python dry_run.py luma         # Luma only
python dry_run.py mlh          # MLH only
python dry_run.py hackiterate  # Hackiterate only
python dry_run.py cv luma      # specific sources with dedup

# Production (requires DB)
python main.py --dry-run     # preview without inserting
python main.py --once        # single crawl pass
python main.py --daemon      # continuous polling (default: 1h)
python main.py --daemon --interval 7200   # poll every 2h
```

## Sources

| Source | Type | Status | Dry-run count |
|---|---|---|---|
| `mlh.com/seasons/{YYYY}/events` | Server-rendered HTML | ✅ Ready | 194 |
| `hackiterate.com/directory` | JS-rendered (Playwright) | ✅ Ready | 6 |
| `cerebralvalley.ai` | Public JSON API (no auth) | ✅ Ready | 9,258 |
| `lu.ma` | Public JSON API (15-city geo sweep) | ✅ Ready | 15 hackathon events |

After cross-source dedup: **~8,976 unique events**.

New sources can be added dynamically via the `scrape_sources` table.

## API Recon Docs

| Platform | Path |
|---|---|
| Cerebral Valley | [cv/API_RECON.md](cv/API_RECON.md) |
| Lu.ma | [luma/API_RECON.md](luma/API_RECON.md) |
| MLH | [mlh/API_RECON.md](mlh/API_RECON.md) |
| Hackiterate | [hackiterate/API_RECON.md](hackiterate/API_RECON.md) |

## Environment

| Variable | Description |
|---|---|
| `DATABASE_URL` | PostgreSQL connection string |
| `PROXY_URL` | Rotating residential proxy (single or comma-separated) |
| `OPENROUTER_API_KEY` | For LLM-based sponsor extraction fallback |
