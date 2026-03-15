"""Cerebral Valley spider — fetches events from the public CV API.

Endpoints (no auth required):
  GET /v1/public/event/pull?featured=true
  GET /v1/public/event/pull?approved=true
"""

import json
import urllib.request
import urllib.error
from datetime import datetime


API_BASE = "https://api.cerebralvalley.ai/v1"

# Status filters to pull — each returns a disjoint set
STATUS_FILTERS = ["featured", "approved"]


def _parse_cv_datetime(dt_str: str | None) -> datetime | None:
    """Parse CV datetime string like '2026-02-14 07:00:00'."""
    if not dt_str:
        return None
    for fmt in ("%Y-%m-%d %H:%M:%S", "%Y-%m-%dT%H:%M:%S"):
        try:
            return datetime.strptime(dt_str, fmt)
        except ValueError:
            continue
    return None


def _fetch_events_by_status(status: str, proxy: str | None = None) -> list[dict]:
    """Fetch events for a given status filter."""
    url = f"{API_BASE}/public/event/pull?{status}=true"
    req = urllib.request.Request(url, method="GET", headers={
        "Accept": "application/json",
        "User-Agent": "RateMyHackathons/1.0",
    })

    if proxy:
        handler = urllib.request.ProxyHandler({"https": proxy, "http": proxy})
        opener = urllib.request.build_opener(handler)
    else:
        opener = urllib.request.build_opener()

    try:
        with opener.open(req, timeout=30) as resp:
            data = json.loads(resp.read().decode())
    except (urllib.error.HTTPError, urllib.error.URLError, OSError) as e:
        print(f"[CV] Failed to fetch {status} events: {e}")
        return []

    return data.get("events", [])


def scrape_cerebralvalley(url: str = "", proxy: str | None = None) -> list[dict]:
    """Scrape Cerebral Valley events from their public API.

    The `url` param is accepted for interface compatibility with main.py
    but the actual API base is hardcoded since it's a JSON API.
    Returns structured event dicts matching the project schema.
    """
    raw_events = []
    seen_ids = set()

    for status in STATUS_FILTERS:
        batch = _fetch_events_by_status(status, proxy=proxy)
        for event in batch:
            eid = event.get("id")
            if eid and eid not in seen_ids:
                seen_ids.add(eid)
                raw_events.append(event)

    events = []
    for raw in raw_events:
        start_dt = _parse_cv_datetime(raw.get("startDateTime"))
        end_dt = _parse_cv_datetime(raw.get("endDateTime"))

        event_url = raw.get("url", "")
        # Ensure it's a full URL
        if event_url and not event_url.startswith("http"):
            event_url = f"https://cerebralvalley.ai/e/{event_url}"

        events.append({
            "name": raw.get("name", "Untitled"),
            "location": raw.get("location"),
            "url": event_url,
            "start_date": start_dt.date() if start_dt else None,
            "end_date": end_dt.date() if end_dt else None,
            "source_url": event_url,
            "source_type": "cerebralvalley",
            "description": raw.get("description"),
            "image_url": raw.get("imageUrl"),
        })

    print(f"[CV] Scraped {len(events)} events ({', '.join(f'{s}' for s in STATUS_FILTERS)})")
    return events
