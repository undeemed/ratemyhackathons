"""Hackiterate spider — scrapes hackiterate.com/directory."""

import re
from datetime import date

from scrapling.fetchers import Fetcher


# Month abbreviation → number
MONTH_MAP = {
    "jan": 1, "feb": 2, "mar": 3, "apr": 4,
    "may": 5, "jun": 6, "jul": 7, "aug": 8,
    "sep": 9, "oct": 10, "nov": 11, "dec": 12,
}


def parse_hackiterate_date(date_text: str) -> tuple[date | None, int | None]:
    """Parse Hackiterate date string like 'FEB 27, 2026'.

    Returns (parsed_date, year).
    """
    date_text = date_text.strip().upper()

    # Pattern: "FEB 27, 2026"
    m = re.match(r"([A-Z]{3})\s+(\d{1,2}),?\s*(\d{4})", date_text)
    if m:
        month = MONTH_MAP.get(m.group(1).lower())
        if month:
            try:
                d = date(int(m.group(3)), month, int(m.group(2)))
                return d, d.year
            except ValueError:
                pass

    return None, None


def scrape_hackiterate(url: str, proxy: str | None = None) -> list[dict]:
    """Scrape Hackiterate directory and return structured event data.

    Args:
        url: Hackiterate directory URL, e.g. https://hackiterate.com/directory
        proxy: Optional proxy URL.

    Returns:
        List of event dicts.
    """
    kwargs = {"stealthy_headers": True}
    if proxy:
        kwargs["proxy"] = proxy

    page = Fetcher.get(url, **kwargs)

    if page.status != 200:
        print(f"[Hackiterate] Failed to fetch {url}: status {page.status}")
        return []

    events = []

    # Hackiterate has event links in the directory listing
    # Each link contains: name, date, status (FINISHED/UPCOMING), location
    event_links = page.css("a[href*='hackiterate.com/']")

    seen_urls = set()

    for link in event_links:
        href = link.attrib.get("href", "")

        # Skip non-event links (navigation, legal, social)
        if not href or any(skip in href for skip in [
            "/directory", "/contact", "/auth", "/legal",
            "twitter.com", "linkedin.com", "instagram.com",
        ]):
            continue

        # Normalize URL
        if not href.startswith("http"):
            href = f"https://hackiterate.com{href}"

        if href in seen_urls:
            continue
        seen_urls.add(href)

        # Extract all text from the link
        texts = [t.strip() for t in link.css("::text").getall() if t.strip()]

        if not texts:
            continue

        # First meaningful text is usually the event name
        name = None
        date_text = ""
        location = None
        status = None

        for t in texts:
            if t in ("VIEW", "FINISHED", "UPCOMING"):
                status = t
                continue
            if re.match(r"^[A-Z]{3}\s+\d{1,2}", t):
                date_text = t
                continue
            if not name and len(t) > 2:
                name = t
                continue
            if name and not location and t not in (name,):
                location = t

        if not name:
            continue

        start_date, _ = parse_hackiterate_date(date_text)

        events.append({
            "name": name,
            "location": location,
            "url": href,
            "start_date": start_date,
            "end_date": None,
            "source_url": href,
            "source_type": "hackiterate",
            "description": None,
            "image_url": None,
        })

    print(f"[Hackiterate] Scraped {len(events)} events from {url}")
    return events
