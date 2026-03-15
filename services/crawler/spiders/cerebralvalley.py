"""Cerebral Valley spider — fetches events from the public CV API.

Endpoints (no auth required):
  GET /v1/public/event/pull?featured=true
  GET /v1/public/event/pull?approved=true
  GET /v1/event/{slug}  — detail endpoint with hosts, media, etc.
"""

import json
import re
import time
import urllib.request
import urllib.error
from datetime import datetime


API_BASE = "https://api.cerebralvalley.ai/v1"

# Status filters to pull — each returns a disjoint set
STATUS_FILTERS = ["featured", "approved"]

# Max events to enrich with host data (detail API calls)
MAX_HOST_ENRICHMENT = 200


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


def _extract_slug(url: str) -> str | None:
    """Extract the event slug from a CV event URL.

    Only works for cerebralvalley.ai URLs (some events link to external sites).
    Examples:
      https://cerebralvalley.ai/e/gemini-3-paris-hackathon?utm_source=...
      → gemini-3-paris-hackathon
    """
    if "cerebralvalley.ai" not in url:
        return None
    m = re.search(r"/e/([^?&#\s]+)", url)
    return m.group(1) if m else None


def _fetch_event_hosts(slug: str, opener) -> list[dict]:
    """Fetch host data from the event detail endpoint."""
    url = f"{API_BASE}/event/{slug}"
    req = urllib.request.Request(url, method="GET", headers={
        "Accept": "application/json",
        "User-Agent": "RateMyHackathons/1.0",
    })
    try:
        with opener.open(req, timeout=15) as resp:
            data = json.loads(resp.read().decode())
    except Exception:
        return []

    hosts_raw = data.get("hosts", [])
    hosts = []
    for h in hosts_raw:
        profile = h.get("userProfile", {})
        name = profile.get("firstName", "")
        last = profile.get("lastName", "")
        if last:
            name = f"{name} {last}".strip()
        if not name:
            continue
        hosts.append({
            "name": name,
            "handle": profile.get("handle"),
            "twitter": profile.get("xHandle"),
            "linkedin": profile.get("linkedinUsername"),
            "github": profile.get("githubUsername"),
            "website": profile.get("siteUrl"),
            "is_org": profile.get("isOrganizationAccount", False),
            "bio": profile.get("description"),
            "role": h.get("role"),
        })
    return hosts


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

    Also enriches events with host/organizer data from the detail endpoint.
    """
    if proxy:
        handler = urllib.request.ProxyHandler({"https": proxy, "http": proxy})
        opener = urllib.request.build_opener(handler)
    else:
        opener = urllib.request.build_opener()

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
    enriched = 0

    for raw in raw_events:
        start_dt = _parse_cv_datetime(raw.get("startDateTime"))
        end_dt = _parse_cv_datetime(raw.get("endDateTime"))

        event_url = raw.get("url", "")
        # Ensure it's a full URL
        if event_url and not event_url.startswith("http"):
            event_url = f"https://cerebralvalley.ai/e/{event_url}"

        # Fetch host data from detail endpoint (rate-limited)
        hosts = []
        slug = _extract_slug(event_url)
        if slug and enriched < MAX_HOST_ENRICHMENT:
            hosts = _fetch_event_hosts(slug, opener)
            enriched += 1
            if enriched % 50 == 0:
                print(f"[CV] Enriched {enriched} events with host data...")
                time.sleep(0.5)  # Be kind to their API

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
            # Enriched fields
            "event_type": raw.get("type"),  # HACKATHON, etc.
            "venue": raw.get("venue"),
            "hosts": hosts,
        })

    host_count = sum(1 for e in events if e.get("hosts"))
    print(f"[CV] Scraped {len(events)} events ({', '.join(STATUS_FILTERS)}), {host_count} with host data")
    return events

