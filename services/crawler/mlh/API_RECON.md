# API Recon — MLH (Major League Hacking)

> Date: 2026-03-14 | Method: Direct HTML scraping

---

## 🟡 Status: HTML SCRAPING (No API)

MLH does not expose a public API. Event data is scraped from the
season schedule page, which is server-rendered HTML.

### Source URL

```
https://mlh.io/seasons/{year}/events
```

Current: `https://mlh.io/seasons/2026/events`

### Data Extraction Method

Standard HTTP fetch + CSS selector parsing via `scrapling.Fetcher`.
No JavaScript rendering required — the page is server-side rendered.

### Selectors

```css
a[href*='utm_source=mlh']   /* Event cards (link to external sites) */
```

Each card contains text fragments in this order:
```
[location, ?badge, name, date, location(dup), mode]
```

Fragments are identified by pattern matching:
- **Date**: matches `^[A-Z]{3}\s+\d` (e.g., `MAR 13 - 15`)
- **Location**: contains `,` (e.g., `Cambridge, MA`)
- **Name**: everything else (longest non-location fragment)

### Date Formats

```
MAR 13 - 15        → same month (start/end)
MAR 28 - APR 1     → different months
```

### Data Available Per Event

- ✅ Event name
- ✅ Start/end dates (parsed from text)
- ✅ Location (city, state/country)
- ✅ Event URL (external, with UTM tracking)
- ✅ Event mode (In-Person, Digital, Hybrid)
- ❌ No description
- ❌ No image URL
- ❌ No sponsor info (only available on individual event pages)
- ❌ No registration count

### Coverage

- **2026 season**: ~100+ events (upcoming + past)
- **Seasons available**: 2013–2026
- Includes collegiate hackathons worldwide

### Spider Strategy

Simple HTTP fetch with `scrapling.Fetcher`. No auth or JS rendering needed.

```python
page = Fetcher.get(url, stealthy_headers=True)
event_links = page.css("a[href*='utm_source=mlh']")
```
