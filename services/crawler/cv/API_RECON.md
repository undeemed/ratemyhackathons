# API Reconnaissance Report — Cerebral Valley & Lu.ma

> Date: 2026-03-13 | Tools: AIDA Exegol + JS bundle analysis + HTTP probing

---

## 🟢 Cerebral Valley — CRACKED (No Auth Required)

### Base URL: `https://api.cerebralvalley.ai/v1`

### 1. Public Event Pull

```
GET /public/event/pull?{status}=true
```

**Status Filters** (one required):
- `featured=true` — Featured/promoted events
- `approved=true` — Community-approved events
- `pending=true` — Pending review
- `denied=true` — Denied events

**Response:**
```json
{
  "detail": "Events retrieved successfully",
  "events": [
    {
      "id": "ac3987b5-...",
      "name": "Gemini 3 Bengaluru Hackathon",
      "description": "Full event description...",
      "descriptionSummary": "AI-generated summary...",
      "startDateTime": "2026-02-14 07:00:00",
      "endDateTime": "2026-02-14 20:00:00",
      "url": "https://cerebralvalley.ai/e/gemini-3-bengaluru-hackathon",
      "location": "Bengaluru, India",
      "venue": null,
      "type": "HACKATHON",
      "status": "featured",
      "imageUrl": "https://cdn.cerebralvalley.ai/...",
      "CVEvent": true,
      "featuredStartTime": "2026-01-27 00:00:00",
      "featuredEndTime": "2026-03-14 00:00:00"
    }
  ]
}
```

### 2. Event Search

```
POST /search/event/search
Content-Type: application/json

{"tz": "America/Los_Angeles", "query": "hackathon"}
```

**Response:**
```json
{
  "detail": "Public event search executed successfully",
  "results": {
    "numTotalMatches": 5,
    "numDirectMatches": 5,
    "numVectorMatches": 0,
    "matches": [
      {
        "data": {
          "id": "36efc3ed-...",
          "name": "Hackathon",
          "startDateTime": "2026-03-19 17:30:00",
          "endDateTime": "2026-03-20 02:30:00",
          "url": "http://hackathon.tempo.xyz",
          "location": "San Francisco, CA",
          "venue": null,
          "imageUrl": null,
          "descriptionSummary": "...",
          "description": "..."
        },
        "metadata": {
          "matchType": "keyword",
          "onlyId": false
        }
      }
    ]
  }
}
```

### 3. Event Detail (auth needed)

```
GET /v1/event/{slug}
```
Returns event detail but requires Clerk JWT auth header.

### Data Available Per Event

- ✅ Event ID (UUID)
- ✅ Name
- ✅ Full description + AI summary
- ✅ Start/end dates
- ✅ Location (city, country)
- ✅ Venue name
- ✅ Event type (HACKATHON, etc.)
- ✅ Status (featured, approved, pending, denied)
- ✅ Image URL (CDN)
- ✅ Event page URL
- ✅ CVEvent flag (own vs community events)
- ✅ Featured start/end times

### Spider Strategy

Simple HTTP with `httpx`/`requests`. No StealthyFetcher needed!

```python
# Pull all featured events
GET https://api.cerebralvalley.ai/v1/public/event/pull?featured=true

# Pull all approved events
GET https://api.cerebralvalley.ai/v1/public/event/pull?approved=true

# Search for specific event types
POST https://api.cerebralvalley.ai/v1/search/event/search
Body: {"tz": "America/Los_Angeles", "query": "hackathon"}
```

### Tech Stack (discovered via JS analysis)
- **Framework**: Next.js (Turbopack)
- **Auth**: Clerk (`pk_live_Y2xlcmsuY2VyZWJyYWx2YWxsZXkuYWkk`)
- **Backend**: Express.js API at `api.cerebralvalley.ai`
- **Database**: Supabase (backend-only, not client-exposed)
- **Analytics**: PostHog
- **Search**: Hybrid keyword + vector search

---

## 🟢 Lu.ma — FULLY OPEN API

### Endpoint: `GET https://api.lu.ma/discover/get-paginated-events`

**Query Parameters:**

| Param | Type | Description |
|---|---|---|
| `pagination_limit` | int | Events per page (default: 20) |
| `next_cursor` | string | Base64 cursor from previous `next_cursor` |
| `geo_latitude` | float | Filter by lat (e.g., `37.7749`) |
| `geo_longitude` | float | Filter by lng (e.g., `-122.4194`) |
| `geo_radius` | float | Radius in km |

**Response:**
```json
{
  "entries": [
    {
      "api_id": "evt-xxx",
      "event": {
        "name": "Event Name",
        "start_at": "2026-03-13T20:00:00.000Z",
        "end_at": "2026-03-14T03:30:00.000Z",
        "timezone": "America/Los_Angeles",
        "url": "rl4zp1hu",
        "cover_url": "https://images.lumacdn.com/...",
        "location_type": "offline",
        "geo_address_info": {
          "city": "San Francisco",
          "full_address": "123 Main St, SF, CA 94103"
        },
        "coordinate": {"latitude": 37.79, "longitude": -122.40}
      },
      "hosts": [{"name": "Host", "twitter_handle": "..."}],
      "guest_count": 36,
      "ticket_info": {"is_free": true, "spots_remaining": 44}
    }
  ],
  "has_more": true,
  "next_cursor": "eyJzdiI6..."
}
```

### Pagination

```
Page 1: GET /discover/get-paginated-events?pagination_limit=50
Page 2: ...&next_cursor={response.next_cursor}
Repeat until has_more === false
```

### Spider Strategy

Simple HTTP. Iterate cursor until `has_more === false`.

---

## 📊 Summary

| Feature | Cerebral Valley | Lu.ma |
|---|---|---|
| API Access | ✅ Public endpoints | ✅ Fully open |
| Auth Required | ❌ No (public routes) | ❌ No |
| Data Format | JSON | JSON |
| Fetcher Needed | `Fetcher` (HTTP) | `Fetcher` (HTTP) |
| Pagination | Status-based batches | Cursor-based |
| Search | ✅ Keyword + vector | ❌ Not available |
| Rate Limits | Unknown | Unknown |
| Data Richness | High (desc + summary) | Very high |

### How We Found Them

1. **CV**: Downloaded all 37 JS bundles → grepped for `fetch()` calls → found `/public/event/pull` and `/search/event/search` → tested with params from error messages
2. **Lu.ma**: Direct API probing at `api.lu.ma` → discovered `/discover/get-paginated-events`
3. **AIDA Exegol**: Used for curl through Docker container
4. **robots.txt**: Revealed `/api/e/og/*` OG image endpoint (Twitterbot-allowed)
