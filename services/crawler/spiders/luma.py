"""Luma spider — fetches events from the public Lu.ma discover API.

Endpoint (no auth required):
  GET https://api.lu.ma/discover/get-paginated-events

Cursor-paginated. Iterates until has_more === false.

NOTE: The discover API has NO server-side category or tag filter.
It returns all event types scoped by geolocation (inferred from IP).
We use geo_latitude/geo_longitude params to sweep major tech hubs,
then filter client-side using keyword matching on event name/description.
"""

import json
import re
import urllib.request
import urllib.error
from datetime import datetime


API_URL = "https://api.lu.ma/discover/get-paginated-events"

# Max pages per city to avoid runaway loops
MAX_PAGES_PER_CITY = 5
PAGE_SIZE = 50

# Keywords that indicate a hackathon/tech event (case-insensitive)
HACKATHON_KEYWORDS = [
    r"\bhack(?:athon|fest|sprint|day|week|night)\b",
    r"\bbuildathon\b",
    r"\bship(?:athon|fest)\b",
    r"\bcode\s*jam\b",
    r"\bcoding\s*(?:challenge|competition|camp)\b",
    r"\bdevcon\b",
    r"\bdev\s*(?:fest|day|summit)\b",
    r"\bai\s*(?:hack|build|jam|sprint|camp)\b",
    r"\bbuild\s*(?:day|night|week|sprint|fest|jam)\b",
    r"\bstartup\s*(?:weekend|hackathon)\b",
    r"\bvibe\s*cod(?:e|ing)\b",
]

_KEYWORD_PATTERN = re.compile("|".join(HACKATHON_KEYWORDS), re.IGNORECASE)

# Major tech hubs to sweep (lat, lng, label)
GEO_CITIES = [
    (37.7749, -122.4194, "San Francisco"),
    (40.7128, -74.0060, "New York"),
    (51.5074, -0.1278, "London"),
    (52.5200, 13.4050, "Berlin"),
    (48.8566, 2.3522, "Paris"),
    (35.6762, 139.6503, "Tokyo"),
    (1.3521, 103.8198, "Singapore"),
    (43.6532, -79.3832, "Toronto"),
    (19.4326, -99.1332, "Mexico City"),
    (-33.8688, 151.2093, "Sydney"),
    (12.9716, 77.5946, "Bangalore"),
    (34.0522, -118.2437, "Los Angeles"),
    (47.6062, -122.3321, "Seattle"),
    (41.8781, -87.6298, "Chicago"),
    (30.2672, -97.7431, "Austin"),
]

GEO_RADIUS = 200  # km — wide enough to catch metro areas


def _is_hackathon_event(name: str, description: str | None = None) -> bool:
    """Check if an event matches hackathon keywords."""
    text = name
    if description:
        text += " " + description
    return bool(_KEYWORD_PATTERN.search(text))


def _parse_luma_datetime(dt_str: str | None) -> datetime | None:
    """Parse Luma ISO datetime like '2026-03-13T20:00:00.000Z'."""
    if not dt_str:
        return None
    for fmt in (
        "%Y-%m-%dT%H:%M:%S.%fZ",
        "%Y-%m-%dT%H:%M:%SZ",
        "%Y-%m-%dT%H:%M:%S",
    ):
        try:
            return datetime.strptime(dt_str, fmt)
        except ValueError:
            continue
    return None


def _build_url(
    cursor: str | None = None,
    lat: float | None = None,
    lng: float | None = None,
) -> str:
    """Build the paginated URL with optional geo params."""
    url = f"{API_URL}?pagination_limit={PAGE_SIZE}"
    if lat is not None and lng is not None:
        url += f"&geo_latitude={lat}&geo_longitude={lng}&geo_radius={GEO_RADIUS}"
    if cursor:
        url += f"&next_cursor={cursor}"
    return url


def scrape_luma(url: str = "", proxy: str | None = None) -> list[dict]:
    """Scrape Luma events from their public discover API.

    Sweeps multiple major tech hubs using geo params since the API
    has no server-side category filter. Deduplicates across cities.

    The `url` param is accepted for interface compatibility with main.py.
    """
    headers = {
        "Accept": "application/json",
        "User-Agent": "RateMyHackathons/1.0",
    }

    if proxy:
        handler = urllib.request.ProxyHandler({"https": proxy, "http": proxy})
        opener = urllib.request.build_opener(handler)
    else:
        opener = urllib.request.build_opener()

    events = []
    seen_urls = set()
    total_seen = 0
    total_pages = 0

    for lat, lng, city_label in GEO_CITIES:
        cursor = None
        city_found = 0

        for page_num in range(MAX_PAGES_PER_CITY):
            req_url = _build_url(cursor, lat=lat, lng=lng)
            req = urllib.request.Request(req_url, method="GET", headers=headers)

            try:
                with opener.open(req, timeout=30) as resp:
                    data = json.loads(resp.read().decode())
            except (urllib.error.HTTPError, urllib.error.URLError, OSError) as e:
                print(f"[Luma] {city_label} page {page_num + 1} failed: {e}")
                break

            entries = data.get("entries", [])
            if not entries:
                break

            new_on_page = 0
            for entry in entries:
                event_data = entry.get("event", {})
                total_seen += 1

                name = event_data.get("name", "Untitled")
                description = event_data.get("description")

                if not _is_hackathon_event(name, description):
                    continue

                event_slug = event_data.get("url", "")
                event_url = f"https://lu.ma/{event_slug}" if event_slug else ""

                if event_url in seen_urls:
                    continue
                seen_urls.add(event_url)

                start_dt = _parse_luma_datetime(event_data.get("start_at"))
                end_dt = _parse_luma_datetime(event_data.get("end_at"))

                geo = event_data.get("geo_address_info") or {}
                location = geo.get("city") or geo.get("full_address")

                # Extract hosts (potential sponsors / organizers)
                hosts_raw = entry.get("hosts", [])
                hosts = [
                    {
                        "name": h.get("name"),
                        "twitter": h.get("twitter_handle"),
                        "linkedin": h.get("linkedin_handle"),
                        "website": h.get("website"),
                        "bio": h.get("bio_short"),
                    }
                    for h in hosts_raw
                    if h.get("name")
                ]

                # Ticket / capacity info
                ticket_raw = entry.get("ticket_info") or {}
                guest_count = entry.get("guest_count")

                events.append({
                    "name": name,
                    "location": location,
                    "url": event_url,
                    "start_date": start_dt.date() if start_dt else None,
                    "end_date": end_dt.date() if end_dt else None,
                    "source_url": event_url,
                    "source_type": "luma",
                    "description": description,
                    "image_url": event_data.get("cover_url"),
                    # Enriched fields from Luma API
                    "timezone": event_data.get("timezone"),
                    "full_address": geo.get("full_address"),
                    "hosts": hosts,
                    "guest_count": guest_count,
                    "is_free": ticket_raw.get("is_free"),
                    "spots_remaining": ticket_raw.get("spots_remaining"),
                    "is_sold_out": ticket_raw.get("is_sold_out"),
                })
                new_on_page += 1
                city_found += 1

            total_pages += 1

            if new_on_page == 0:
                break
            if not data.get("has_more", False):
                break
            cursor = data.get("next_cursor")
            if not cursor:
                break

        if city_found:
            print(f"[Luma] {city_label}: {city_found} hackathon events")

    print(f"[Luma] {len(events)} unique hackathon events from {total_seen} total across {len(GEO_CITIES)} cities ({total_pages} pages)")
    return events


