"""Sponsor extraction — follows event URLs to scrape sponsor names.

Strategy (in priority order):
1. Find sections with class/id containing 'sponsor' or 'partner'
   → extract img alt text from within those sections
2. Find headings containing 'sponsor'/'partner'
   → extract img alt text from the parent container
3. Find images whose src path contains '/sponsor'
   → use alt text as sponsor name
4. LLM fallback — if CSS strategies find 0 sponsors, send
   relevant page text to a free OpenRouter model
"""

import re

from scrapling.fetchers import StealthyFetcher, Fetcher
from llm import ask

# Words that appear as img alt but aren't sponsor names
NOISE_ALTS = {
    "logo", "icon", "background", "hero", "banner", "arrow",
    "close", "menu", "search", "chevron", "decoration",
    "decorative", "waves", "gradient", "texture", "bridge",
}

# Patterns in alt text that indicate non-sponsors
NOISE_PATTERNS = re.compile(
    r"^(a group of|a large group|textureGradient|knot decorative"
    r"|waves|bridge|hero|banner|background)",
    re.IGNORECASE,
)

LLM_SYSTEM = (
    "You extract sponsor and partner company names from hackathon event pages. "
    "Return ONLY a JSON array of company name strings. "
    "Include sponsors, partners, and supporting organizations. "
    "Exclude individual people, speakers, organizers, social media links, "
    "and the hackathon itself. "
    "If no sponsors found, return []."
)

LLM_PROMPT_TEMPLATE = (
    "Extract all sponsor and partner company names from this hackathon page text.\n"
    "Return ONLY a JSON array of strings.\n\n"
    "PAGE TEXT:\n{text}"
)


def _clean_alt(alt: str) -> str | None:
    """Return cleaned sponsor name or None if it's noise."""
    alt = alt.strip()
    if not alt or len(alt) < 2 or len(alt) > 60:
        return None
    if alt.lower() in NOISE_ALTS:
        return None
    if NOISE_PATTERNS.match(alt):
        return None
    # Skip generic descriptions
    if alt.lower().startswith(("a ", "an ", "the ", "image of")):
        return None
    return alt


def _extract_sponsors_llm(page_text: str) -> list[str]:
    """Use LLM to extract sponsor names from page text."""
    # Smart extraction: grab text around sponsor/partner mentions
    keywords = ["sponsor", "partner", "powered by", "supported by", "backed by"]
    chunks = []
    lower = page_text.lower()

    for kw in keywords:
        idx = 0
        while True:
            pos = lower.find(kw, idx)
            if pos < 0:
                break
            # Grab 1500 chars before and after the keyword
            start = max(0, pos - 500)
            end = min(len(page_text), pos + 2000)
            chunks.append(page_text[start:end])
            idx = pos + len(kw)

    if chunks:
        # Deduplicate overlapping chunks
        text = "\n---\n".join(chunks[:5])
    else:
        # No keyword matches — send first 6k chars as fallback
        text = page_text[:6000]

    # Cap at 8k chars total
    text = text[:8000]
    prompt = LLM_PROMPT_TEMPLATE.format(text=text)

    response = ask(prompt, system=LLM_SYSTEM, max_tokens=512)
    if not response:
        return []

    # Parse JSON array from response
    try:
        # Handle markdown code blocks
        cleaned = response.strip()
        if cleaned.startswith("```"):
            cleaned = cleaned.split("\n", 1)[1] if "\n" in cleaned else cleaned
            cleaned = cleaned.rsplit("```", 1)[0]
        cleaned = cleaned.strip()

        result = __import__("json").loads(cleaned)
        if isinstance(result, list):
            return [s.strip() for s in result if isinstance(s, str) and s.strip()]
    except Exception:
        pass

    return []


def scrape_sponsors(url: str, proxy: str | None = None) -> list[str]:
    """Fetch an event page and extract sponsor/partner company names.

    Uses StealthyFetcher (Playwright) since most hackathon sites
    are JS-rendered SPAs. Falls back to LLM extraction if CSS
    strategies find nothing.

    Returns:
        List of unique sponsor name strings.
    """
    try:
        kwargs = {}
        if proxy:
            kwargs["proxy"] = {"server": proxy}

        page = StealthyFetcher.fetch(
            url,
            headless=True,
            **kwargs,
        )
    except Exception as e:
        print(f"  [SPONSORS] Failed to fetch {url}: {e}")
        return []

    if page.status != 200:
        return []

    sponsors = set()

    # --- Strategy 1: Sections with sponsor/partner class/id ---
    for sel in [
        "[class*='sponsor' i]", "[id*='sponsor' i]",
        "[class*='partner' i]", "[id*='partner' i]",
    ]:
        try:
            sections = page.css(sel)
            for section in sections:
                for img in section.css("img[alt]"):
                    name = _clean_alt(img.attrib.get("alt", ""))
                    if name:
                        sponsors.add(name)
        except Exception:
            pass

    # --- Strategy 2: Headings with sponsor/partner text ---
    for heading in page.css("h1, h2, h3, h4, h5, h6"):
        try:
            texts = heading.css("::text").getall()
            heading_text = " ".join(
                t.strip() for t in texts if t.strip()
            ).lower()
        except Exception:
            heading_text = (heading.text or "").lower()

        if any(w in heading_text for w in [
            "sponsor", "partner", "supported by", "backed by",
            "powered by",
        ]):
            # Get images from the heading's parent container
            parent = heading.parent
            if parent:
                for img in parent.css("img[alt]"):
                    name = _clean_alt(img.attrib.get("alt", ""))
                    if name:
                        sponsors.add(name)

    # --- Strategy 3: Images with /sponsor in src path ---
    for img in page.css("img[alt]"):
        src = img.attrib.get("src", "")
        if "/sponsor" in src.lower() or "/partner" in src.lower():
            name = _clean_alt(img.attrib.get("alt", ""))
            if name:
                sponsors.add(name)

    # --- Strategy 4: LLM fallback ---
    if not sponsors:
        # Gather page text for LLM
        try:
            all_text = " ".join(
                t.strip()
                for t in page.css("::text").getall()
                if t.strip()
            )
        except Exception:
            all_text = page.text or ""

        if all_text and len(all_text) > 50:
            llm_sponsors = _extract_sponsors_llm(all_text)
            if llm_sponsors:
                print(
                    f"  [SPONSORS-LLM] Found {len(llm_sponsors)} "
                    f"via LLM fallback"
                )
                sponsors.update(llm_sponsors)

    return sorted(sponsors)
