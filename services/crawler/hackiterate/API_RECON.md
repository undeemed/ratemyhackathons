# API Recon — Hackiterate

> Date: 2026-03-14 | Method: Playwright (StealthyFetcher) scraping

---

## 🟡 Status: JS RENDERING REQUIRED (No API)

Hackiterate is a Next.js SPA that client-renders all content. No public API
is exposed. Event data must be scraped using Playwright (StealthyFetcher)
to execute JavaScript before DOM access.

### Source URL

```
https://hackiterate.com/directory
```

### Data Extraction Method

JavaScript rendering via `scrapling.StealthyFetcher` (Playwright headless).
Falls back to `scrapling.Fetcher` if StealthyFetcher is unavailable, but
this will likely return an empty page.

```python
page = StealthyFetcher.fetch(
    url,
    headless=True,
    wait_selector="a[href]",
    wait_selector_state="attached",
)
```

### Tech Stack

- **Framework**: Next.js (with `dpl_` deployment IDs → Vercel)
- **Rendering**: Fully client-side (RSC payloads, no SSR HTML)
- **Auth**: Clerk (login page at `/auth/login`)
- **Analytics**: Vercel SpeedInsights

### Directory Page Structure

Each event card is an `<a>` link containing text fragments:
```
[name, date, status, location(s), "VIEW"]
```

- **Name**: Longest non-location, non-status fragment
- **Date**: matches `^[A-Z]{3}\s+\d{1,2}` (e.g., `FEB 27, 2026`)
- **Status**: `FINISHED`, `UPCOMING`, or `LIVE`
- **Location**: City names or multi-city lists

### Detail Pages

Also fully client-rendered. The slug pattern `/{slug}` maps to event detail.
No SSR content is available — would require a second Playwright render
to extract sponsors, description, and participant info.

### Data Available Per Event (directory)

- ✅ Event name
- ✅ Start date (parsed from text)
- ❌ End date (not shown in directory)
- ✅ Location (city or multi-city list)
- ✅ Event URL
- ✅ Status (finished/upcoming/live)
- ❌ Description (detail page only, JS-rendered)
- ❌ Image URL (not in directory cards)
- ❌ Sponsors (detail page only, JS-rendered)

### Coverage

- **Current events**: ~6 hackathons (small, curated platform)
- Focuses on "selective and highly technical hackathons"
- Partners include Mistral AI for worldwide hack series

### Spider Strategy

Playwright (StealthyFetcher) is required for the directory page.
Extracting sponsors/descriptions would require a second pass visiting
each detail page — currently not implemented.
