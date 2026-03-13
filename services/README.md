# Services

Standalone services that complement the main backend API. Each runs independently and shares the same PostgreSQL database.

## Overview

```mermaid
graph TB
    subgraph Services
        Crawler[Crawler<br/>Python + Scrapling]
        Analytics[Analytics<br/>Rust + SvelteKit]
    end

    subgraph External
        MLH[mlh.com]
        Hackiterate[hackiterate.com]
        CV[cerebralvalley.ai]
        Luma[luma.com]
    end

    subgraph Infrastructure
        DB[(PostgreSQL)]
        Proxy[Rotating Residential<br/>Proxy 80m+ IPs]
    end

    Crawler -->|scrape| MLH
    Crawler -->|scrape| Hackiterate
    Crawler -.->|future| CV
    Crawler -.->|future| Luma
    Crawler -->|via| Proxy
    Crawler -->|write events| DB
    Analytics -->|read stats| DB
    Analytics -->|SSE live feed| Browser[Dashboard :5174]
```

## Services

| Service | Language | Port | Description |
|---|---|---|---|
| [`crawler/`](crawler/) | Python | CLI | Scrapes hackathon listings from MLH, Hackiterate, etc. |
| [`analytics/`](analytics/) | Rust + Svelte | `:8081` / `:5174` | Live analytics dashboard with crawl stats |

## Running All Services

```bash
# Terminal 1 — Main API
cd ../backend && cargo run                    # :8080

# Terminal 2 — Crawler (one-shot)
cd crawler && python main.py --once

# Terminal 3 — Analytics API
cd analytics && cargo run                     # :8081

# Terminal 4 — Analytics Dashboard
cd analytics/dashboard && bun dev             # :5174
```
