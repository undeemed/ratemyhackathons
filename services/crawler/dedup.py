"""Deduplication logic: hash-based + fuzzy name matching + cross-source URL normalization."""

import hashlib
import re
from urllib.parse import urlparse, parse_qs, urlencode, urlunparse


def make_source_hash(source_type: str, source_url: str) -> str:
    """SHA-256 hash of (source_type, source_url) for primary dedup."""
    raw = f"{source_type}:{source_url}"
    return hashlib.sha256(raw.encode()).hexdigest()


def normalize_event_url(url: str) -> str:
    """Normalize an event URL to its canonical form for cross-source dedup.

    - Strips UTM params, tracking params, and fragments
    - Normalizes lu.ma / luma.com domain variants
    - Strips trailing slashes
    - Lowercases the domain

    Examples:
      https://lu.ma/abc?utm_source=cv → https://lu.ma/abc
      https://luma.com/abc            → https://lu.ma/abc
      https://cerebralvalley.ai/e/foo?utm_source=cv-events → https://cerebralvalley.ai/e/foo
    """
    if not url:
        return ""

    parsed = urlparse(url)
    domain = parsed.netloc.lower()

    # Normalize luma.com → lu.ma
    if domain in ("luma.com", "www.luma.com"):
        domain = "lu.ma"
    elif domain == "www.lu.ma":
        domain = "lu.ma"

    # Strip tracking params
    tracking_prefixes = ("utm_", "ref", "fbclid", "gclid", "mc_", "source")
    params = parse_qs(parsed.query, keep_blank_values=False)
    clean_params = {
        k: v for k, v in params.items()
        if not any(k.lower().startswith(p) for p in tracking_prefixes)
    }

    # Rebuild URL without fragment
    clean_query = urlencode(clean_params, doseq=True) if clean_params else ""
    path = parsed.path.rstrip("/") or "/"

    normalized = urlunparse((
        parsed.scheme or "https",
        domain,
        path,
        "",          # params
        clean_query,
        "",          # fragment
    ))
    return normalized


def normalize_name(name: str) -> str:
    """Normalize event name for fuzzy comparison."""
    # Lowercase, strip whitespace, remove special chars
    name = name.lower().strip()
    name = re.sub(r"[^a-z0-9\s]", "", name)
    name = re.sub(r"\s+", " ", name)
    return name


def levenshtein_ratio(s1: str, s2: str) -> float:
    """Compute normalized Levenshtein similarity (0.0 to 1.0)."""
    if s1 == s2:
        return 1.0
    len1, len2 = len(s1), len(s2)
    if len1 == 0 or len2 == 0:
        return 0.0

    # Wagner-Fischer algorithm
    matrix = [[0] * (len2 + 1) for _ in range(len1 + 1)]
    for i in range(len1 + 1):
        matrix[i][0] = i
    for j in range(len2 + 1):
        matrix[0][j] = j

    for i in range(1, len1 + 1):
        for j in range(1, len2 + 1):
            cost = 0 if s1[i - 1] == s2[j - 1] else 1
            matrix[i][j] = min(
                matrix[i - 1][j] + 1,      # deletion
                matrix[i][j - 1] + 1,      # insertion
                matrix[i - 1][j - 1] + cost,  # substitution
            )

    distance = matrix[len1][len2]
    max_len = max(len1, len2)
    return 1.0 - (distance / max_len)


def is_fuzzy_duplicate(name: str, existing_names: list[str], threshold: float = 0.85) -> str | None:
    """Check if name fuzzy-matches any existing name. Returns matched name or None."""
    normalized = normalize_name(name)
    for existing in existing_names:
        existing_norm = normalize_name(existing)
        if levenshtein_ratio(normalized, existing_norm) >= threshold:
            return existing
    return None


def deduplicate_cross_source(all_events: list[dict]) -> list[dict]:
    """Deduplicate events across sources using normalized URLs.

    When multiple sources provide the same event (e.g., CV links to a lu.ma
    event that was also scraped directly from Luma), keeps the richer record
    and merges source information.

    Priority: luma > cerebralvalley > mlh > hackiterate
    (Luma has the richest data: hosts, tickets, guest count)
    """
    SOURCE_PRIORITY = {
        "luma": 0,
        "cerebralvalley": 1,
        "mlh": 2,
        "hackiterate": 3,
    }

    # Group events by normalized URL
    url_groups: dict[str, list[dict]] = {}
    for event in all_events:
        norm_url = normalize_event_url(event.get("url", ""))
        if not norm_url:
            # No URL — can't dedup by URL, keep as-is
            url_groups.setdefault("__no_url__" + event.get("name", ""), []).append(event)
            continue
        url_groups.setdefault(norm_url, []).append(event)

    deduped = []
    merged_count = 0

    for norm_url, group in url_groups.items():
        if len(group) == 1:
            deduped.append(group[0])
            continue

        # Multiple sources for the same event — pick the richest one
        group.sort(key=lambda e: SOURCE_PRIORITY.get(e.get("source_type", ""), 99))
        primary = group[0]

        # Merge hosts from all sources (CV might have org hosts that Luma doesn't)
        all_hosts = []
        seen_host_names = set()
        for event in group:
            for host in event.get("hosts", []):
                hname = host.get("name", "")
                if hname and hname not in seen_host_names:
                    seen_host_names.add(hname)
                    all_hosts.append(host)
        if all_hosts:
            primary["hosts"] = all_hosts

        # Keep the longer description
        for event in group:
            desc = event.get("description")
            if desc and (not primary.get("description") or len(desc) > len(primary.get("description", ""))):
                primary["description"] = desc

        # Track all sources
        primary["all_sources"] = [
            {"source_type": e.get("source_type"), "source_url": e.get("source_url")}
            for e in group
        ]

        deduped.append(primary)
        merged_count += 1

    if merged_count:
        print(f"[DEDUP] Merged {merged_count} cross-source duplicates → {len(deduped)} unique events")

    return deduped

