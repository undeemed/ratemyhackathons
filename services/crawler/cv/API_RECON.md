# API Recon — Cerebral Valley

> Date: 2026-03-13 | Tools: AIDA Exegol + JS bundle analysis + HTTP probing

---

## 🟢 Status: CRACKED (No Auth Required)

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

Simple HTTP with `urllib`. No StealthyFetcher needed!

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

### How We Found It

1. Downloaded all 37 JS bundles → grepped for `fetch()` calls
2. Found `/public/event/pull` and `/search/event/search`
3. Tested with params from error messages
4. AIDA Exegol used for curl through Docker container
