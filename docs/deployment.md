# Self-Host RateMyHackathons on Mac Mini via Tailscale + OrbStack

## Context

The project runs entirely on localhost today. The goal is to deploy the full stack — PostgreSQL, Rust backend, Python crawler, Rust analytics, SvelteKit frontend — to a Mac Mini via Tailscale, using OrbStack (Docker-compatible) containers for isolation. All deployment files live in a self-contained `deploy/` directory.

## Target Machine

- **Host**: `lexies-mac-mini.tailc4c9b.ts.net` (Tailscale MagicDNS)
- **IP**: `100.77.36.51` (Tailscale)
- **Access**: SSH over Tailscale
- **Runtime**: OrbStack (Docker-compatible)

## Architecture

```
Mac Mini (via OrbStack Docker Compose)
┌──────────────────────────────────────────┐
│  docker compose up -d                    │
│  ├── postgres:17-alpine  :5432 → :5432   │
│  ├── backend (Rust)      :8080 → :8080   │
│  ├── crawler (Python)    daemon, no port │
│  ├── analytics (Rust)    :8081 → :8081   │
│  └── frontend (SvelteKit):3000 → :3000   │
│                                          │
│  Volume: pgdata (persistent DB storage)  │
└──────────────────────────────────────────┘
```

All ports bound to `0.0.0.0` — accessible over Tailscale from dev machine.

## Quick Start

```bash
# 1. Create .env in deploy/ with your secrets
cp deploy/.env.example deploy/.env
# Edit deploy/.env with your POSTGRES_PASSWORD, etc.

# 2. Deploy to Mac Mini
cd deploy
./deploy.sh
```

## Services

| Service | Port | Image | Description |
|---------|------|-------|-------------|
| postgres | 5432 | postgres:17-alpine | PostgreSQL database with persistent volume |
| backend | 8080 | Custom (Rust multi-stage) | Actix-Web REST API |
| crawler | — | Custom (Python + Playwright) | Continuous event scraper |
| analytics | 8081 | Custom (Rust multi-stage) | Analytics API + SSE |
| frontend | 3000 | Custom (Bun build + Node) | SvelteKit web app |

## Configuration

All configuration is via environment variables in `deploy/.env`:

```
POSTGRES_PASSWORD=<your-password>
OPENROUTER_API_KEY=<optional, for LLM sponsor extraction>
PROXY_URL=<optional, for crawler proxy>
```

## Database

PostgreSQL data persists in a Docker volume (`pgdata`). Migrations run automatically on first boot via `docker-entrypoint-initdb.d`.

To connect from dev machine:
```bash
psql -h lexies-mac-mini.tailc4c9b.ts.net -U rmh -d ratemyhackathons
```

## Verification

```bash
# Check all containers are running
ssh lexies-mac-mini.tailc4c9b.ts.net "cd ~/ratemyhackathons/deploy && docker compose ps"

# Backend health
curl http://lexies-mac-mini.tailc4c9b.ts.net:8080/health

# Database
psql -h lexies-mac-mini.tailc4c9b.ts.net -U rmh -d ratemyhackathons -c "SELECT count(*) FROM events;"

# Crawler logs
ssh lexies-mac-mini.tailc4c9b.ts.net "cd ~/ratemyhackathons/deploy && docker compose logs crawler"

# Frontend
open http://lexies-mac-mini.tailc4c9b.ts.net:3000
```

## Updating

```bash
# From dev machine — syncs code and rebuilds containers
cd deploy && ./deploy.sh
```

## Resetting Database

```bash
ssh lexies-mac-mini.tailc4c9b.ts.net "cd ~/ratemyhackathons/deploy && docker compose down -v && docker compose up -d"
```

The `-v` flag removes the `pgdata` volume, and migrations re-run on next boot.
