"""Deduplication logic: hash-based + fuzzy name matching."""

import hashlib


def make_source_hash(source_type: str, source_url: str) -> str:
    """SHA-256 hash of (source_type, source_url) for primary dedup."""
    raw = f"{source_type}:{source_url}"
    return hashlib.sha256(raw.encode()).hexdigest()


def normalize_name(name: str) -> str:
    """Normalize event name for fuzzy comparison."""
    import re
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
