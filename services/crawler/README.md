# Crawler

Python scraping service that continuously polls hackathon sources, deduplicates against existing events, and auto-detects sponsoring companies.

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
        +normalize_name(name) str
        +levenshtein_ratio(s1, s2) float
        +is_fuzzy_duplicate(name, existing) str?
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

    Main --> Db
    Main --> Dedup
    Main --> Proxy
    Main --> Company
    Main --> MLHSpider
    Main --> HackiterateSpider
```

## Crawl Pipeline

```mermaid
flowchart LR
    A[Fetch HTML] --> B{Hash exists?}
    B -->|Yes| C[Skip]
    B -->|No| D{Fuzzy name match?}
    D -->|>0.85| E[Link to existing event]
    D -->|No match| F[Insert new event]
    F --> G[Detect companies]
    G --> H[Link companies]
```

## Spider Architecture

```mermaid
flowchart TB
    subgraph MLHSpider["MLH Spider"]
        M1["Fetch mlh.com/seasons/YYYY/events"] --> M2["CSS: a with utm_source"]
        M2 --> M3["Extract name via h3::text"]
        M2 --> M4["Parse date: MAR 13 - 15"]
        M2 --> M5["Extract location via comma text"]
        M2 --> M6["Strip UTM → clean event URL"]
        M3 & M4 & M5 & M6 --> M7["Yield event dict"]
    end

    subgraph HackiterateSpider["Hackiterate Spider"]
        H1["Fetch hackiterate.com/directory"] --> H2["CSS: a with hackiterate.com href"]
        H2 --> H3["Filter nav/social links"]
        H3 --> H4["Extract name from first text"]
        H3 --> H5["Parse date: FEB 27, 2026"]
        H3 --> H6["Detect FINISHED / UPCOMING"]
        H4 & H5 & H6 --> H7["Yield event dict"]
    end

    subgraph Future["Future Spiders"]
        F1["Cerebral Valley"] -.-> F3["StealthyFetcher"]
        F2["Luma"] -.-> F3
        F3 -.-> F4["Reverse-engineered API"]
    end

    M7 & H7 --> Pipeline["Dedup + Insert Pipeline"]
    F4 -.-> Pipeline
```

## Usage

```bash
# Setup
uv venv && uv pip install -r requirements.txt
cp .env.example .env   # set DATABASE_URL, PROXY_URL

# Run
python main.py --dry-run     # preview without inserting
python main.py --once        # single crawl pass
python main.py --daemon      # continuous polling (default: 1h)
python main.py --daemon --interval 7200   # poll every 2h
```

## Sources

| Source | Type | Status |
|---|---|---|
| `mlh.com/seasons/{YYYY}/events` | Server-rendered | ✅ Ready |
| `hackiterate.com/directory` | Server-rendered | ✅ Ready |
| `cerebralvalley.ai` | SPA (Next.js) | 🔜 Needs API reverse-engineering |
| `luma.com` | SPA | 🔜 Needs API reverse-engineering |

New sources can be added dynamically via the `scrape_sources` table.

## Environment

| Variable | Description |
|---|---|
| `DATABASE_URL` | PostgreSQL connection string |
| `PROXY_URL` | Rotating residential proxy (single or comma-separated) |
