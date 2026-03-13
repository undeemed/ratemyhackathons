"""MLH spider — scrapes mlh.com/seasons/{year}/events."""

import re
from datetime import date

from scrapling.fetchers import Fetcher


# Month abbreviation → number
MONTH_MAP = {
    "jan": 1, "feb": 2, "mar": 3, "apr": 4,
    "may": 5, "jun": 6, "jul": 7, "aug": 8,
    "sep": 9, "oct": 10, "nov": 11, "dec": 12,
}

SKIP_TEXTS = {"In-Person", "Digital", "Hybrid", "DIVERSITY", ""}


def parse_mlh_date(date_text: str, year: int) -> tuple[date | None, date | None]:
    """Parse MLH date strings like 'MAR 13 - 15' or 'MAR 28 - APR 1'.

    Returns (start_date, end_date).
    """
    date_text = date_text.strip().upper()

    # Pattern: "MAR 13 - 15" (same month)
    m = re.match(r"([A-Z]{3})\s+(\d{1,2})\s*-\s*(\d{1,2})", date_text)
    if m:
        month = MONTH_MAP.get(m.group(1).lower())
        if month:
            try:
                start = date(year, month, int(m.group(2)))
                end = date(year, month, int(m.group(3)))
                return start, end
            except ValueError:
                pass

    # Pattern: "MAR 28 - APR 1" (different months)
    m = re.match(
        r"([A-Z]{3})\s+(\d{1,2})\s*-\s*([A-Z]{3})\s+(\d{1,2})", date_text
    )
    if m:
        month1 = MONTH_MAP.get(m.group(1).lower())
        month2 = MONTH_MAP.get(m.group(3).lower())
        if month1 and month2:
            try:
                start = date(year, month1, int(m.group(2)))
                end = date(year, month2, int(m.group(4)))
                return start, end
            except ValueError:
                pass

    return None, None


def extract_year_from_url(url: str) -> int:
    """Extract the season year from an MLH URL."""
    m = re.search(r"/seasons/(\d{4})/", url)
    if m:
        return int(m.group(1))
    return date.today().year


def _is_date(t: str) -> bool:
    return bool(re.match(r"^[A-Z]{3}\s+\d", t.upper()))


def _is_location(t: str) -> bool:
    return "," in t


def scrape_mlh(url: str, proxy: str | None = None) -> list[dict]:
    """Scrape MLH events page and return structured event data.

    MLH card text order observed:
      [location, ?badge, name, date, location(dup), mode]
    We identify each fragment by pattern, then what's left is the name.
    """
    kwargs = {"stealthy_headers": True}
    if proxy:
        kwargs["proxy"] = proxy

    page = Fetcher.get(url, **kwargs)

    if page.status != 200:
        print(f"[MLH] Failed to fetch {url}: status {page.status}")
        return []

    year = extract_year_from_url(url)
    events = []

    event_links = page.css("a[href*='utm_source=mlh']")
    seen_urls = set()

    for link in event_links:
        href = link.attrib.get("href", "")
        if not href or "utm_source" not in href:
            continue

        event_url = href.split("?")[0] if "?" in href else href
        if event_url in seen_urls:
            continue
        seen_urls.add(event_url)

        texts = [t.strip() for t in link.css("::text").getall() if t.strip()]

        # Categorise each text fragment
        date_text = ""
        location = None
        name_candidates = []

        for t in texts:
            if t in SKIP_TEXTS:
                continue
            if _is_date(t):
                date_text = t
            elif _is_location(t) and location is None:
                location = t
            elif t not in SKIP_TEXTS:
                name_candidates.append(t)

        # The real event name is typically the longest non-location fragment
        name = None
        for c in name_candidates:
            if not _is_location(c) and not _is_date(c):
                name = c
                break

        if not name:
            continue

        start_date, end_date = parse_mlh_date(date_text, year)

        events.append({
            "name": name,
            "location": location,
            "url": event_url,
            "start_date": start_date,
            "end_date": end_date,
            "source_url": href,
            "source_type": "mlh",
            "description": None,
            "image_url": None,
        })

    print(f"[MLH] Scraped {len(events)} events from {url}")
    return events
