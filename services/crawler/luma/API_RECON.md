# API Recon — Lu.ma

> Date: 2026-03-14 | Tools: Direct API probing at `api.lu.ma`

---

## 🟢 Status: FULLY OPEN (No Auth Required)

### Endpoint: `GET https://api.lu.ma/discover/get-paginated-events`

### Query Parameters

| Param | Type | Description |
|---|---|---|
| `pagination_limit` | int | Events per page (default: 20, max: 50) |
| `next_cursor` | string | Base64 cursor from previous response |
| `geo_latitude` | float | Filter by lat (e.g., `37.7749`) |
| `geo_longitude` | float | Filter by lng (e.g., `-122.4194`) |
| `geo_radius` | float | Radius in km (e.g., `200`) |

### ⚠️ No Server-Side Category/Tag Filter

Exhaustively tested 20+ param names:
`category`, `tag`, `discover_tag_id`, `discover_tag_slug`, `discover_place_id`,
`discover_place_slug`, `topic`, `slug`, `type`, `label`, `k`, `p`, `t`, `filters`, `city`, `place`, `geo_place_slug`, `discover_topic`

**All ignored** — the API returns the same geo-scoped results regardless.
Lu.ma's `/ai`, `/tech`, `/sf` browse pages do filtering entirely client-side in React.

### Spider Strategy

Multi-city geo sweep across 15 major tech hubs + client-side keyword filtering.

### Response Structure

```json
{
  "entries": [
    {
      "api_id": "evt-xxx",
      "event": {
        "name": "World Model Hackathon",
        "start_at": "2026-03-14T00:00:00.000Z",
        "end_at": "2026-03-16T03:30:00.000Z",
        "timezone": "America/Los_Angeles",
        "url": "worldsinaction-sf26",
        "cover_url": "https://images.lumacdn.com/...",
        "location_type": "offline",
        "event_type": "independent",
        "visibility": "public",
        "geo_address_info": {
          "city": "San Francisco",
          "city_state": "San Francisco, California",
          "full_address": "Founders, Inc., 2 Marina Blvd...",
          "country": "United States"
        },
        "coordinate": {"latitude": 37.8069, "longitude": -122.4318},
        "waitlist_enabled": false
      },
      "hosts": [
        {
          "name": "XR Bootcamp",
          "twitter_handle": "XR_Bootcamp",
          "linkedin_handle": "/in/...",
          "website": "https://xrbootcamp.com/",
          "bio_short": "Building XR experiences"
        }
      ],
      "guest_count": 752,
      "ticket_count": 752,
      "ticket_info": {
        "is_free": false,
        "is_sold_out": false,
        "spots_remaining": 248,
        "is_near_capacity": false,
        "require_approval": true,
        "price": null,
        "max_price": null
      },
      "featured_guests": [
        {
          "name": "Nicholas Lin",
          "bio_short": "Technical Founder | 4x Hackathon Winner",
          "twitter_handle": null,
          "linkedin_handle": "/in/..."
        }
      ]
    }
  ],
  "has_more": true,
  "next_cursor": "eyJzdiI6..."
}
```

### Data Available Per Event

- ✅ Event name, description (sometimes null in list)
- ✅ Start/end times (ISO 8601) + timezone
- ✅ URL slug → `https://lu.ma/{slug}`
- ✅ Cover image URL (CDN)
- ✅ Location: city, full address, coordinates, country
- ✅ **Hosts** (name, twitter, linkedin, website, bio) — potential sponsors
- ✅ **Guest count** (registered attendees)
- ✅ **Ticket info** (free/paid, spots remaining, sold out, approval required)
- ✅ Featured guests (top attendees with profiles)
- ✅ Event type (independent, etc.)
- ✅ Waitlist status
- ❌ No event category/tag field
- ❌ Description often null in list (only on detail page)

### Pagination

```
Page 1: GET /discover/get-paginated-events?pagination_limit=50&geo_latitude=37.77&geo_longitude=-122.41&geo_radius=200
Page 2: ...&next_cursor={response.next_cursor}
Repeat until has_more === false
```

### How We Found It

1. Direct API probing at `api.lu.ma`
2. Discovered `/discover/get-paginated-events`
3. Brute-forced 20+ param names for category/city filtering — none work
4. Confirmed: filtering is client-side only in the React SPA
