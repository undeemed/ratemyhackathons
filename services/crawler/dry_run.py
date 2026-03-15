#!/usr/bin/env python3
"""Standalone dry-run — test spiders without a database.

Usage:
    python dry_run.py              # all sources with cross-source dedup
    python dry_run.py cv           # Cerebral Valley only
    python dry_run.py luma         # Luma only
    python dry_run.py mlh          # MLH only
    python dry_run.py hackiterate  # Hackiterate only
    python dry_run.py cv luma      # specific sources with dedup
"""

import sys

from spiders.cerebralvalley import scrape_cerebralvalley
from spiders.luma import scrape_luma
from spiders.mlh import scrape_mlh
from spiders.hackiterate import scrape_hackiterate
from dedup import deduplicate_cross_source


SPIDERS = {
    "cv": ("Cerebral Valley", scrape_cerebralvalley),
    "luma": ("Lu.ma", scrape_luma),
    "mlh": ("MLH", lambda: scrape_mlh("https://mlh.io/seasons/2026/events")),
    "hackiterate": ("Hackiterate", lambda: scrape_hackiterate("https://hackiterate.com/directory")),
}


def print_events(label: str, events: list[dict]):
    """Pretty-print extracted events."""
    print(f"\n{'='*70}")
    print(f"  {label} — {len(events)} events")
    print(f"{'='*70}")

    for i, ev in enumerate(events, 1):
        print(f"\n  [{i:3d}] {ev['name']}")
        print(f"        📅  {ev.get('start_date', '?')} → {ev.get('end_date', '?')}")
        print(f"        📍  {ev.get('location') or 'N/A'}")
        print(f"        🔗  {ev.get('url') or 'N/A'}")
        if ev.get("hosts"):
            hosts = ev["hosts"]
            host_names = [h.get("name", "?") for h in hosts[:5]]
            extra = f" +{len(hosts)-5} more" if len(hosts) > 5 else ""
            print(f"        🏢  {', '.join(host_names)}{extra}")
        if ev.get("all_sources"):
            sources = [s["source_type"] for s in ev["all_sources"]]
            print(f"        📦  Sources: {' + '.join(sources)}")
        if ev.get("guest_count"):
            print(f"        👥  {ev['guest_count']} guests")
        if ev.get("image_url"):
            print(f"        🖼️   {ev['image_url'][:80]}...")

    print(f"\n{'─'*70}")
    print(f"  Total: {len(events)} events from {label}")
    print(f"{'─'*70}\n")


def main():
    targets = sys.argv[1:] if len(sys.argv) > 1 else list(SPIDERS.keys())

    # Validate targets
    for key in targets:
        if key not in SPIDERS:
            print(f"Unknown source: '{key}'. Choose from: {', '.join(SPIDERS.keys())}")
            sys.exit(1)

    all_events = []

    for key in targets:
        label, scraper = SPIDERS[key]
        print(f"\n⏳ Fetching from {label}...")

        try:
            events = scraper()
            all_events.extend(events)

            if len(targets) == 1:
                print_events(label, events)
        except Exception as e:
            print(f"❌ {label} failed: {e}")
            import traceback
            traceback.print_exc()

    # Cross-source dedup when running multiple sources
    if len(targets) > 1 and all_events:
        print(f"\n⏳ Cross-source deduplication...")
        print(f"   Before: {len(all_events)} total events")
        deduped = deduplicate_cross_source(all_events)
        print(f"   After:  {len(deduped)} unique events")
        print_events("All Sources (deduped)", deduped)


if __name__ == "__main__":
    main()

