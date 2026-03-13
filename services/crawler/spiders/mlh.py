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
    m = re.match(r"([A-Z]{3})\s+(\d{1,2})\s*-\s*([A-Z]{3})\s+(\d{1,2})", date_text)
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


def scrape_mlh(url: str, proxy: str | None = None) -> list[dict]:
    """Scrape MLH events page and return structured event data.

    Args:
        url: Full MLH season events URL, e.g. https://mlh.com/seasons/2026/events
        proxy: Optional proxy URL.

    Returns:
        List of event dicts with keys: name, location, url, start_date, end_date, source_url
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

    # MLH uses anchor tags wrapping each event card
    # Each card has the event name, date, and location
    event_links = page.css("a[href*='utm_source=mlh']")

    seen_names = set()

    for link in event_links:
        # Extract event URL
        href = link.attrib.get("href", "")
        if not href or "utm_source" not in href:
            continue

        # Get the actual event URL (strip UTM params for clean URL)
        event_url = href.split("?")[0] if "?" in href else href

        # Extract event name — look for the heading text
        name = None
        # Try multiple selectors for the event name
        name_el = link.css("h3::text")
        if name_el:
            name = name_el.get("").strip()

        if not name:
            # Fallback: look for any prominent text
            texts = link.css("::text").getall()
            # Filter out short fragments and dates
            for t in texts:
                t = t.strip()
                if len(t) > 3 and not re.match(r"^[A-Z]{3}\s+\d", t) and t not in ("In-Person", "Digital", "Hybrid"):
                    name = t
                    break

        if not name or name in seen_names:
            continue
        seen_names.add(name)

        # Extract date
        date_text = ""
        for t in link.css("::text").getall():
            t = t.strip()
            if re.match(r"^[A-Z]{3}\s+\d", t):
                date_text = t
                break

        start_date, end_date = parse_mlh_date(date_text, year)

        # Extract location
        location = None
        for t in link.css("::text").getall():
            t = t.strip()
            if t and t not in (name, date_text, "In-Person", "Digital", "Hybrid", "") and "," in t:
                location = t
                break

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
