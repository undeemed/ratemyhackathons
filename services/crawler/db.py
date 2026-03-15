"""Database operations for the crawler."""

import asyncpg
import json
import uuid
import hashlib

from uuid_utils import uuid7
from datetime import date, datetime, timezone


def _json_default(obj):
    """JSON serializer for date/datetime objects."""
    if isinstance(obj, (date, datetime)):
        return obj.isoformat()
    raise TypeError(f"Object of type {type(obj).__name__} is not JSON serializable")


async def get_pool(database_url: str) -> asyncpg.Pool:
    """Create a connection pool."""
    return await asyncpg.create_pool(database_url, min_size=1, max_size=5)


async def get_enabled_sources(pool: asyncpg.Pool) -> list[dict]:
    """Fetch all enabled scrape sources."""
    rows = await pool.fetch(
        """
        SELECT id, name, source_type, base_url, poll_interval_hours, last_polled_at
        FROM scrape_sources
        WHERE enabled = true
        ORDER BY source_type, name
        """
    )
    return [dict(r) for r in rows]


async def should_poll(pool: asyncpg.Pool, source_id: uuid.UUID, interval_hours: int) -> bool:
    """Check if a source is due for polling based on its interval."""
    row = await pool.fetchrow(
        "SELECT last_polled_at FROM scrape_sources WHERE id = $1",
        source_id,
    )
    if row is None or row["last_polled_at"] is None:
        return True
    elapsed = datetime.now(timezone.utc) - row["last_polled_at"]
    return elapsed.total_seconds() >= interval_hours * 3600


async def mark_polled(pool: asyncpg.Pool, source_id: uuid.UUID):
    """Update last_polled_at timestamp for a source."""
    await pool.execute(
        "UPDATE scrape_sources SET last_polled_at = NOW() WHERE id = $1",
        source_id,
    )


async def hash_exists(pool: asyncpg.Pool, source_hash: str) -> bool:
    """Check if a crawl source hash already exists (dedup)."""
    row = await pool.fetchrow(
        "SELECT 1 FROM crawl_sources WHERE source_hash = $1",
        source_hash,
    )
    return row is not None


async def insert_event(pool: asyncpg.Pool, event: dict) -> uuid.UUID:
    """Insert a new event and return its ID."""
    event_id = uuid7()
    await pool.execute(
        """
        INSERT INTO events (id, name, description, location, url, start_date, end_date, image_url)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        """,
        event_id,
        event["name"],
        event.get("description"),
        event.get("location"),
        event.get("url"),
        event.get("start_date"),
        event.get("end_date"),
        event.get("image_url"),
    )
    return event_id


async def insert_crawl_source(
    pool: asyncpg.Pool,
    event_id: uuid.UUID | None,
    source_url: str,
    source_type: str,
    raw_data: dict | None,
    source_hash: str,
):
    """Record a crawl source entry with dedup hash."""
    crawl_id = uuid7()

    await pool.execute(
        """
        INSERT INTO crawl_sources (id, event_id, source_url, source_type, raw_data, source_hash)
        VALUES ($1, $2, $3, $4, $5::jsonb, $6)
        ON CONFLICT (source_hash) DO NOTHING
        """,
        crawl_id,
        event_id,
        source_url,
        source_type,
        json.dumps(raw_data, default=_json_default) if raw_data else None,
        source_hash,
    )


async def link_company_to_event(pool: asyncpg.Pool, event_id: uuid.UUID, company_id: uuid.UUID):
    """Link a company to an event (ignore if already linked)."""
    await pool.execute(
        """
        INSERT INTO event_companies (event_id, company_id)
        VALUES ($1, $2)
        ON CONFLICT DO NOTHING
        """,
        event_id,
        company_id,
    )


async def get_all_companies(pool: asyncpg.Pool) -> list[dict]:
    """Fetch all companies for name matching."""
    rows = await pool.fetch("SELECT id, name FROM companies ORDER BY name")
    return [dict(r) for r in rows]


async def get_or_create_company(
    pool: asyncpg.Pool,
    name: str,
    website: str | None = None,
    description: str | None = None,
) -> uuid.UUID:
    """Find a company by name or create it. Returns the company ID."""
    row = await pool.fetchrow(
        "SELECT id FROM companies WHERE LOWER(name) = LOWER($1)",
        name.strip(),
    )
    if row:
        return row["id"]

    company_id = uuid7()
    await pool.execute(
        """
        INSERT INTO companies (id, name, website, description, search_vector)
        VALUES ($1, $2, $3, $4, to_tsvector('english', $2 || ' ' || COALESCE($4, '')))
        ON CONFLICT DO NOTHING
        """,
        company_id,
        name.strip(),
        website,
        description,
    )
    return company_id


async def find_events_by_name_window(
    pool: asyncpg.Pool, name: str, start_date, days_window: int = 30
) -> list[dict]:
    """Find existing events with similar dates for fuzzy matching."""
    rows = await pool.fetch(
        """
        SELECT id, name, start_date
        FROM events
        WHERE start_date BETWEEN ($1::date - $2 * INTERVAL '1 day')
                          AND ($1::date + $2 * INTERVAL '1 day')
        """,
        start_date,
        days_window,
    )
    return [dict(r) for r in rows]


async def update_event_coords(
    pool, event_id, latitude: float, longitude: float
):
    """Update an event's geocoded coordinates."""
    await pool.execute(
        "UPDATE events SET latitude = $1, longitude = $2 WHERE id = $3",
        latitude,
        longitude,
        event_id,
    )
