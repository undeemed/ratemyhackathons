"""Hackathon crawler — main entry point.

Usage:
    python main.py --once        # single crawl pass
    python main.py --daemon      # continuous loop
    python main.py --dry-run     # scrape but don't insert (preview)
"""

import argparse
import asyncio
import os
import sys

from dotenv import load_dotenv

import db
from dedup import make_source_hash, is_fuzzy_duplicate
from proxy import get_single_proxy
from company import detect_companies
from sponsors import scrape_sponsors
from spiders.mlh import scrape_mlh
from spiders.hackiterate import scrape_hackiterate


# Source type → scraper function
SCRAPERS = {
    "mlh": scrape_mlh,
    "hackiterate": scrape_hackiterate,
}


async def process_events(
    pool,
    events: list[dict],
    source_type: str,
    proxy: str | None = None,
    dry_run: bool = False,
):
    """Process scraped events: dedup, insert, link companies."""
    companies = await db.get_all_companies(pool)

    inserted = 0
    skipped_hash = 0
    skipped_fuzzy = 0

    for event in events:
        source_url = event.get("source_url", event.get("url", ""))
        source_hash = make_source_hash(source_type, source_url)

        # 1. Hash-based dedup
        if await db.hash_exists(pool, source_hash):
            skipped_hash += 1
            continue

        # 2. Fuzzy name dedup (if we have a start_date)
        if event.get("start_date"):
            existing = await db.find_events_by_name_window(
                pool, event["name"], event["start_date"]
            )
            existing_names = [e["name"] for e in existing]
            matched = is_fuzzy_duplicate(event["name"], existing_names)
            if matched:
                # Link crawl source to existing event but don't create duplicate
                existing_event = next(e for e in existing if e["name"] == matched)
                if not dry_run:
                    await db.insert_crawl_source(
                        pool,
                        event_id=existing_event["id"],
                        source_url=source_url,
                        source_type=source_type,
                        raw_data=event,
                        source_hash=source_hash,
                    )
                print(
                    f"  [FUZZY] '{event['name']}' matches "
                    f"existing '{matched}' — linked, not duplicated"
                )
                skipped_fuzzy += 1
                continue

        # 3. Insert new event
        if dry_run:
            print(f"  [DRY] Would insert: {event['name']} ({event.get('start_date', '?')})")
            inserted += 1
            continue

        event_id = await db.insert_event(pool, event)
        await db.insert_crawl_source(
            pool,
            event_id=event_id,
            source_url=source_url,
            source_type=source_type,
            raw_data=event,
            source_hash=source_hash,
        )

        # 4. Best-effort company detection (regex on name/desc)
        text_to_scan = " ".join(filter(None, [
            event.get("name", ""),
            event.get("description", ""),
        ]))
        matched_companies = detect_companies(text_to_scan, companies)
        for company in matched_companies:
            await db.link_company_to_event(pool, event_id, company["id"])
            print(
                f"  [COMPANY] Linked '{company['name']}'"
                f" to '{event['name']}'"
            )

        # 5. Scrape sponsors from event's own website
        event_url = event.get("url", "")
        if event_url:
            try:
                sponsor_names = scrape_sponsors(event_url, proxy=proxy)
                if sponsor_names:
                    print(
                        f"  [SPONSORS] {event['name']}: "
                        f"{len(sponsor_names)} found"
                    )
                    # Store scraped sponsors in the event's raw data
                    event["scraped_sponsors"] = sponsor_names
                    for sname in sponsor_names:
                        print(f"    • {sname}")
            except (ConnectionError, OSError, ValueError) as e:
                print(
                    f"  [SPONSORS] Failed for {event['name']}: {e}"
                )

        inserted += 1
        print(f"  [NEW] {event['name']} ({event.get('start_date', '?')})")

    print(f"  Summary: {inserted} new, {skipped_hash} hash-skipped, {skipped_fuzzy} fuzzy-skipped")
    return inserted


async def crawl_once(pool, dry_run: bool = False):
    """Run a single crawl pass across all enabled sources."""
    sources = await db.get_enabled_sources(pool)
    if not sources:
        print("No enabled sources found. Add entries to scrape_sources table.")
        return

    proxy = get_single_proxy()
    total_inserted = 0

    for source in sources:
        source_type = source["source_type"]
        base_url = source["base_url"]
        source_id = source["id"]
        interval = source["poll_interval_hours"]

        # Check if source is due for polling
        if not dry_run and not await db.should_poll(pool, source_id, interval):
            print(f"[SKIP] {source['name']} — polled recently (interval: {interval}h)")
            continue

        scraper = SCRAPERS.get(source_type)
        if not scraper:
            print(f"[WARN] No scraper for source_type='{source_type}', skipping {source['name']}")
            continue

        print(f"\n{'='*60}")
        print(f"[CRAWL] {source['name']} ({source_type})")
        print(f"  URL: {base_url}")
        print(f"{'='*60}")

        try:
            events = scraper(base_url, proxy=proxy)
            inserted = await process_events(
                pool, events, source_type,
                proxy=proxy, dry_run=dry_run,
            )
            total_inserted += inserted

            if not dry_run:
                await db.mark_polled(pool, source_id)
        except (ConnectionError, OSError, ValueError) as e:
            print(f"[ERROR] {source['name']}: {e}")
            continue

    print(f"\n{'='*60}")
    print(f"Crawl complete. {total_inserted} events processed.")
    print(f"{'='*60}")


async def daemon_loop(pool, interval_seconds: int = 3600):
    """Run crawls in a continuous loop."""
    print(f"Daemon mode: crawling every {interval_seconds}s")
    while True:
        await crawl_once(pool)
        print(f"\nSleeping {interval_seconds}s until next crawl...")
        await asyncio.sleep(interval_seconds)


async def async_main(args):
    """Async entry point."""
    load_dotenv()

    database_url = os.getenv("DATABASE_URL")
    if not database_url:
        print("ERROR: DATABASE_URL not set. Create a .env file or set the env var.")
        sys.exit(1)

    pool = await db.get_pool(database_url)

    try:
        if args.daemon:
            await daemon_loop(pool, interval_seconds=args.interval)
        else:
            await crawl_once(pool, dry_run=args.dry_run)
    finally:
        await pool.close()


def cli():
    """CLI entry point."""
    parser = argparse.ArgumentParser(
        description="RateMyHackathons event crawler",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python main.py --once              Single crawl pass
  python main.py --dry-run           Preview without inserting
  python main.py --daemon            Continuous polling
  python main.py --daemon --interval 7200   Poll every 2h
        """,
    )
    parser.add_argument("--once", action="store_true", default=True, help="Run a single crawl pass (default)")
    parser.add_argument("--daemon", action="store_true", help="Run in continuous daemon mode")
    parser.add_argument("--dry-run", action="store_true", help="Scrape but don't write to DB")
    parser.add_argument(
        "--interval", type=int, default=3600,
        help="Daemon poll interval in seconds (default: 3600)",
    )

    args = parser.parse_args()

    asyncio.run(async_main(args))


if __name__ == "__main__":
    cli()
