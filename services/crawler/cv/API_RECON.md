# API Recon — Cerebral Valley

> Date: 2026-03-14 | Tools: AIDA Exegol + JS bundle analysis + HTTP probing

---

## 🟢 Status: CRACKED (No Auth Required)

### Base URL: `https://api.cerebralvalley.ai/v1`

### 1. Public Event Pull

```
GET /public/event/pull?{status}=true
```

**Status Filters** (one required):
- `featured=true` — Featured/promoted events (~9 events)
- `approved=true` — Community-approved events (~9,200+ events)
- `pending=true` — Pending review
- `denied=true` — Denied events

**Response:**
```json
{
  "detail": "Events retrieved successfully",
  "events": [
    {
      "id": "ac3987b5-...",
      "name": "Gemini 3 Paris Hackathon",
      "description": "Full event description...",
      "descriptionSummary": "AI-generated summary...",
      "startDateTime": "2026-02-14 07:00:00",
      "endDateTime": "2026-02-14 20:00:00",
      "url": "https://cerebralvalley.ai/e/gemini-3-paris-hackathon",
      "location": "Paris",
      "venue": null,
      "type": "HACKATHON",
      "status": "featured",
      "imageUrl": "https://cdn.cerebralvalley.ai/...",
      "CVEvent": true
    }
  ]
}
```

### 2. Event Detail (with hosts!)

```
GET /event/{slug}
```

**No auth required.** Returns the full event with host profiles.

```json
{
  "event": { "...same fields as pull..." },
  "hosts": [
    {
      "role": "co-host",
      "userProfile": {
        "firstName": "Google DeepMind",
        "lastName": "",
        "handle": "deepmind",
        "isOrganizationAccount": true,
        "xHandle": "googledeepmind",
        "linkedinUsername": "...",
        "githubUsername": "...",
        "siteUrl": null,
        "description": "...",
        "avatarUrl": "https://cdn.cerebralvalley.ai/..."
      }
    }
  ],
  "questions": [...],
  "media": [...]
}
```

**Key fields per host:**
- `firstName` + `lastName` → display name
- `isOrganizationAccount` → distinguishes companies from individuals
- `xHandle` → Twitter/X username
- `linkedinUsername`, `githubUsername`, `siteUrl`
- `handle` → CV profile slug
- `role` → "co-host", etc.

### 3. Event Search

```
POST /search/event/search
Content-Type: application/json

{"tz": "America/Los_Angeles", "query": "hackathon"}
```

Returns same basic fields as pull (no hosts).

### Data Available Per Event

| Field | Pull | Detail |
|---|---|---|
| Name, description, AI summary | ✅ | ✅ |
| Start/end dates | ✅ | ✅ |
| Location, venue | ✅ | ✅ |
| Event type (HACKATHON, etc.) | ✅ | ✅ |
| Image URL | ✅ | ✅ |
| **Hosts/organizers** | ❌ | ✅ |
| **Host socials** (Twitter, LinkedIn, GitHub) | ❌ | ✅ |
| **isOrganizationAccount** | ❌ | ✅ |
| Media (videos, images) | ❌ | ✅ |
| Registration questions | ❌ | ✅ |

### ⚠️ URL Overlap with Luma

67% of approved events (6,238/9,258) link to external platforms:
- `lu.ma` / `luma.com`: 6,238 events
- `partiful.com`: 1,048 events
- `eventbrite.com`: 553 events
- `meetup.com`: 304 events

Only ~30 events are CV-hosted (`cerebralvalley.ai/e/...`).
Cross-source dedup via URL normalization is required.

### Spider Strategy

1. Pull featured + approved events via public endpoint
2. Enrich CV-hosted events with host data via detail endpoint
3. Deduplicate against Luma events by normalized URL

### Tech Stack (discovered via JS analysis)
- **Framework**: Next.js (Turbopack)
- **Auth**: Clerk
- **Backend**: Express.js API at `api.cerebralvalley.ai`
- **Database**: Supabase
- **Analytics**: PostHog
- **Search**: Hybrid keyword + vector search
