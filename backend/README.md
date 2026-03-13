# Backend API

REST API server for RateMyHackathons — handles events, companies, users, reviews, and search.

## Architecture

```mermaid
classDiagram
    class Main {
        +main()
        +health_check()
    }
    class Config {
        +database_url: String
        +from_env()
    }
    class Db {
        +init_pool() PgPool
    }
    class Routes {
        +PaginatedResponse~T~
        +PaginationParams
    }
    class EventRoutes {
        +list_events()
        +get_event()
        +create_event()
    }
    class CompanyRoutes {
        +list_companies()
        +get_company()
        +create_company()
    }
    class UserRoutes {
        +list_users()
        +get_user()
        +create_user()
    }
    class ReviewRoutes {
        +create_review()
        +get_review()
        +create_review_comment()
        +vote_review()
    }
    class SearchRoutes {
        +search()
    }

    Main --> Config
    Main --> Db
    Main --> Routes
    Routes --> EventRoutes
    Routes --> CompanyRoutes
    Routes --> UserRoutes
    Routes --> ReviewRoutes
    Routes --> SearchRoutes
```

## Request Flow

```mermaid
sequenceDiagram
    participant Client
    participant Actix as Actix-Web
    participant Validate as Validator
    participant Sanitize as Ammonia
    participant DB as PostgreSQL

    Client->>Actix: POST /api/events
    Actix->>Validate: body.validate()
    alt Validation fails
        Validate-->>Client: 400 Bad Request
    end
    Actix->>Sanitize: ammonia::clean(fields)
    Actix->>DB: INSERT INTO events
    DB-->>Actix: EventRow
    Actix-->>Client: 201 Created (JSON)
```

## Stack

| Crate | Purpose |
|---|---|
| `actix-web` | HTTP server |
| `sqlx` | Async Postgres queries |
| `validator` | Declarative input validation |
| `ammonia` | HTML sanitization (XSS prevention) |
| `uuid` | UUIDv7 generation |
| `chrono` | Timestamps |

## Running

```bash
cp .env.example .env   # set DATABASE_URL
psql -d ratemyhackathons -f migrations/20260313_initial_schema.sql
psql -d ratemyhackathons -f migrations/20260313_review_votes_comments.sql
psql -d ratemyhackathons -f migrations/20260313_user_profiles_event_slugs.sql
psql -d ratemyhackathons -f migrations/20260313_crawl_registry.sql
cargo run   # http://127.0.0.1:8080
cargo test  # 45 tests
```

## Endpoints

| Method | Path | Description |
|---|---|---|
| `GET` | `/health` | Health check |
| `GET/POST` | `/api/events` | List / create events |
| `GET` | `/api/events/{id}` | Event detail + companies + reviews |
| `GET/POST` | `/api/companies` | List / create companies |
| `GET` | `/api/companies/{id}` | Company detail + events |
| `GET/POST` | `/api/users` | List / create users |
| `GET` | `/api/users/{id}` | User detail + reviews |
| `POST` | `/api/reviews` | Create review |
| `GET` | `/api/reviews/{id}` | Review + votes + threaded comments |
| `POST` | `/api/reviews/{id}/vote` | Vote helpful/unhelpful |
| `POST` | `/api/reviews/{id}/comments` | Add comment (threaded) |
| `GET` | `/api/search?q=` | Full-text search |
