"""Hackiterate spider — scrapes hackiterate.com/directory.

Hackiterate is a JS-rendered SPA, so we use StealthyFetcher (Playwright)
to get the fully rendered page. Falls back to the static Fetcher if
StealthyFetcher is unavailable.
"""

import re
from datetime import date

try:
    from scrapling.fetchers import StealthyFetcher
    HAS_STEALTH = True
except ImportError:
    from scrapling.fetchers import Fetcher
    HAS_STEALTH = False


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

    Uses StealthyFetcher (Playwright) because Hackiterate renders
    its directory listing client-side with JavaScript.
    """
    kwargs = {}
    if proxy:
        kwargs["proxy"] = {"server": proxy}

    if HAS_STEALTH:
        page = StealthyFetcher.fetch(
            url,
            headless=True,
            wait_selector="a[href]",
            wait_selector_state="attached",
            **kwargs,
        )
    else:
        print("[Hackiterate] StealthyFetcher not available, using Fetcher")
        if proxy:
            kwargs = {"proxy": proxy}
        page = Fetcher.get(url, stealthy_headers=True, **kwargs)

    if page.status != 200:
        print(f"[Hackiterate] Failed to fetch {url}: status {page.status}")
        return []

    events = []

    # Get all links on the page
    all_links = page.css("a[href]")
    seen_urls = set()

    for link in all_links:
        href = link.attrib.get("href", "")

        # Only care about links that look like event detail pages
        # Skip navigation, social, auth links
        if not href:
            continue
        if any(skip in href for skip in [
            "/directory", "/contact", "/auth", "/legal",
            "twitter.com", "linkedin.com", "instagram.com",
            "github.com", "mailto:", "#", "javascript:",
        ]):
            continue

        # Normalize URL
        if href.startswith("/"):
            href = f"https://hackiterate.com{href}"

        if not href.startswith("http"):
            continue

        # Must be hackiterate.com event page
        if "hackiterate.com" not in href:
            continue

        if href in seen_urls:
            continue
        seen_urls.add(href)

        # Extract all text from the link container
        texts = []
        for node in link.css("*"):
            t = node.text
            if t and t.strip():
                texts.append(t.strip())
        # Also get direct text
        if link.text and link.text.strip():
            texts.insert(0, link.text.strip())

        # Deduplicate while preserving order
        seen_texts = set()
        unique_texts = []
        for t in texts:
            if t not in seen_texts:
                seen_texts.add(t)
                unique_texts.append(t)
        texts = unique_texts

        if not texts:
            continue

        # Categorize fragments
        name = None
        date_text = ""
        location = None
        status = None

        for t in texts:
            if t.upper() in ("VIEW", "FINISHED", "UPCOMING", "LIVE"):
                status = t.upper()
                continue
            if re.match(r"^[A-Z]{3}\s+\d{1,2}", t.upper()):
                date_text = t
                continue
            if not name and len(t) > 2:
                name = t
                continue
            if name and not location and t != name and len(t) > 2:
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
