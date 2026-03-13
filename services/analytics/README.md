# Analytics

Rust API + SvelteKit dashboard for monitoring crawl activity, event trends, and review analytics.

## Architecture

```mermaid
classDiagram
    class Server {
        +main()
        -pool: PgPool
        -bind: String
    }
    class Db {
        +crawl_stats(pool) CrawlStats
        +crawl_history(pool, days) Vec~CrawlDay~
        +crawl_sources(pool) Vec~SourceInfo~
        +trending_events(pool, days, limit) Vec~TrendingEvent~
        +events_timeline(pool) Vec~EventWeek~
        +rating_distribution(pool) Vec~RatingBucket~
        +recent_reviews(pool, limit) Vec~RecentReview~
    }
    class CrawlRoutes {
        +stats()
        +history()
        +sources()
    }
    class EventRoutes {
        +trending()
        +timeline()
    }
    class ReviewRoutes {
        +stats()
        +recent()
    }
    class LiveRoute {
        +stream() SSE
    }

    Server --> Db
    Server --> CrawlRoutes
    Server --> EventRoutes
    Server --> ReviewRoutes
    Server --> LiveRoute
```

## Data Flow

```mermaid
sequenceDiagram
    participant Dashboard as SvelteKit :5174
    participant API as Rust API :8081
    participant DB as PostgreSQL

    Dashboard->>API: GET /api/crawl/stats
    API->>DB: SELECT COUNT(*) FROM crawl_sources ...
    DB-->>API: {total, last_24h, ...}
    API-->>Dashboard: JSON

    Dashboard->>API: GET /api/live (SSE)
    loop Every 5s
        API->>DB: Check new counts
        API-->>Dashboard: data: {"type":"crawl","count":3}
    end
```

## API Endpoints

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/crawl/stats` | Total scraped, 24h/7d/30d breakdown |
| `GET` | `/api/crawl/history?days=30` | Crawls per day time-series |
| `GET` | `/api/crawl/sources` | Source registry + poll status |
| `GET` | `/api/events/trending?days=30` | Most reviewed events |
| `GET` | `/api/events/timeline` | Events per week/month |
| `GET` | `/api/reviews/stats` | Rating distribution (1-5) |
| `GET` | `/api/reviews/recent?limit=10` | Latest reviews feed |
| `GET` | `/api/live` | SSE real-time event stream |

## Dashboard

SvelteKit app with Tailwind v4 and LayerChart.

| Panel | Description |
|---|---|
| Stat Cards | Total scraped, last 24h/7d/30d |
| Source Health | Table with poll status, event counts, enable dots |
| Live Feed | SSE-powered scrolling feed of new events + reviews |
| Rating Distribution | Horizontal bar chart (1-5 stars) |
| Trending Events | Ranked list with review counts + avg rating |
| Recent Reviews | Latest reviews with star rating + author |

```mermaid
flowchart TB
    subgraph Layout["+layout.svelte"]
        Header["Header: Logo + LIVE dot"]
    end

    subgraph Page["+page.svelte"]
        Cards["Stat Cards ×4
        total / 24h / 7d / 30d"]
        SourceTable["Source Health Table
        name / type / count / poll / status"]
        Live["Live Feed
        SSE → scrolling event list"]
        Ratings["Rating Distribution
        horizontal bar chart 1-5★"]
        Trending["Trending Events
        ranked list + avg rating"]
        Recent["Recent Reviews
        stars + author + timeAgo"]
    end

    subgraph Data["Data Layer"]
        API["api.ts fetch helpers"]
        SSE["EventSource /api/live"]
    end

    Layout --> Page
    Cards & SourceTable & Ratings & Trending & Recent --> API
    Live --> SSE
    API -->|proxy :5174 → :8081| Rust["Rust API"]
    SSE -->|text/event-stream| Rust
```

## Running

```bash
# API server
cargo run   # http://127.0.0.1:8081

# Dashboard (separate terminal)
cd dashboard
bun install
bun dev     # http://localhost:5174
```

## Stack

| Layer | Tech |
|---|---|
| API | Rust + Actix-Web |
| Dashboard | SvelteKit + Tailwind v4 |
| Charts | LayerChart (Svelte-native D3) |
| Real-time | Server-Sent Events (SSE) |
