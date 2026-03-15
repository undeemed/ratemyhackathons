"""Hackiterate spider — scrapes hackiterate.com/directory.

Hackiterate embeds structured JSON in its Next.js RSC payload,
so a plain HTTP fetch is sufficient — no Playwright needed.
"""

import json
import re
from datetime import date, datetime

from scrapling.fetchers import Fetcher


def scrape_hackiterate(url: str, proxy: str | None = None) -> list[dict]:
    """Scrape Hackiterate directory from the RSC payload."""
    kwargs = {}
    if proxy:
        kwargs["proxy"] = proxy

    page = Fetcher.get(url, stealthy_headers=True, **kwargs)

    if page.status != 200:
        print(f"[Hackiterate] Failed to fetch {url}: status {page.status}")
        return []

    html = page.get_all_text() if hasattr(page, 'get_all_text') else str(page)

    # Try to get raw HTML body
    try:
        html = page.body.decode() if isinstance(page.body, bytes) else page.body
    except Exception:
        html = str(page.text) if hasattr(page, 'text') else str(page)

    # Extract hackathon JSON from Next.js RSC push payload
    chunks = re.findall(r'self\.__next_f\.push\(\[1,"(.*?)"\]\)', html)

    hackathons = []
    for chunk in chunks:
        try:
            unescaped = chunk.encode().decode('unicode_escape')
        except Exception:
            continue

        m = re.search(r'"hackathons":(\[.*)', unescaped)
        if not m:
            continue

        try:
            decoder = json.JSONDecoder()
            hackathons, _ = decoder.raw_decode(m.group(1))
            break
        except json.JSONDecodeError:
            continue

    if not hackathons:
        print(f"[Hackiterate] No hackathon data found in RSC payload from {url}")
        return []

    events = []
    for h in hackathons:
        start_date = None
        end_date = None

        if h.get("start_date"):
            try:
                start_date = datetime.fromisoformat(h["start_date"]).date()
            except (ValueError, TypeError):
                pass

        if h.get("end_date"):
            try:
                end_date = datetime.fromisoformat(h["end_date"]).date()
            except (ValueError, TypeError):
                pass

        slug = h.get("slug", "")
        event_url = f"https://hackiterate.com/{slug}" if slug else url

        events.append({
            "name": h.get("name", ""),
            "location": h.get("location"),
            "url": event_url,
            "start_date": start_date,
            "end_date": end_date,
            "source_url": event_url,
            "source_type": "hackiterate",
            "description": h.get("description"),
            "image_url": h.get("banner_url"),
        })

    print(f"[Hackiterate] Scraped {len(events)} events from {url}")
    return events
