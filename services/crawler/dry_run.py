#!/usr/bin/env python3
"""Standalone dry-run — test CV and Luma spiders without a database.

Usage:
    python dry_run.py              # both sources
    python dry_run.py cv           # Cerebral Valley only
    python dry_run.py luma         # Luma only
"""

import sys

from spiders.cerebralvalley import scrape_cerebralvalley
from spiders.luma import scrape_luma


SPIDERS = {
    "cv": ("Cerebral Valley", scrape_cerebralvalley),
    "luma": ("Lu.ma", scrape_luma),
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
        if ev.get("description"):
            desc = ev["description"][:120].replace("\n", " ")
            print(f"        📝  {desc}{'...' if len(ev['description']) > 120 else ''}")
        if ev.get("image_url"):
            print(f"        🖼️   {ev['image_url'][:80]}...")

    print(f"\n{'─'*70}")
    print(f"  Total: {len(events)} events from {label}")
    print(f"{'─'*70}\n")


def main():
    targets = sys.argv[1:] if len(sys.argv) > 1 else list(SPIDERS.keys())

    for key in targets:
        if key not in SPIDERS:
            print(f"Unknown source: '{key}'. Choose from: {', '.join(SPIDERS.keys())}")
            continue

        label, scraper = SPIDERS[key]
        print(f"\n⏳ Fetching from {label}...")

        try:
            events = scraper()
            print_events(label, events)
        except Exception as e:
            print(f"❌ {label} failed: {e}")
            import traceback
            traceback.print_exc()


if __name__ == "__main__":
    main()
