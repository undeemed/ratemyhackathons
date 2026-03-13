"""Best-effort company detection from event text."""

import re


def detect_companies(text: str, known_companies: list[dict]) -> list[dict]:
    """Scan text for known company names and return matches.

    Args:
        text: Event description, sponsor section, or page text.
        known_companies: List of dicts with 'id' and 'name' from DB.

    Returns:
        List of matched company dicts.
    """
    if not text or not known_companies:
        return []

    text_lower = text.lower()
    matches = []

    for company in known_companies:
        name = company["name"]
        # Case-insensitive word-boundary match
        pattern = r"\b" + re.escape(name.lower()) + r"\b"
        if re.search(pattern, text_lower):
            matches.append(company)

    return matches
