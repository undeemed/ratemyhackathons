"""Proxy setup for Scrapling fetchers."""

import os
from scrapling.fetchers import ProxyRotator


def get_proxy_rotator() -> ProxyRotator | None:
    """Create a ProxyRotator from PROXY_URL env var.

    PROXY_URL can be:
      - A single proxy: http://user:pass@host:port
      - Comma-separated: http://p1:8080,http://p2:8080
    """
    proxy_url = os.getenv("PROXY_URL", "").strip()
    if not proxy_url:
        return None

    proxies = [p.strip() for p in proxy_url.split(",") if p.strip()]
    if not proxies:
        return None

    return ProxyRotator(proxies)


def get_single_proxy() -> str | None:
    """Get a single proxy URL for one-off StealthyFetcher requests."""
    proxy_url = os.getenv("PROXY_URL", "").strip()
    if not proxy_url:
        return None
    # Return the first proxy if comma-separated
    return proxy_url.split(",")[0].strip() or None
