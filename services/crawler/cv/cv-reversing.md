# Cerebral Valley — JS Reverse Engineering Report

> Date: 2026-03-13 | Source: 37 JS bundles (3.3MB) from Next.js/Turbopack build
> Method: AIDA Exegol + grep analysis

---

## TL;DR

- **63 unique API endpoints** across 3 services discovered
- **LinkedIn profile lookup without auth** — query anyone by username
- **227 hackathon submissions exposed** — full team data, no auth
- **Messaging WebSocket** at `wss://{host}/messaging` — separate microservice
- **No Supabase credentials** in client JS — backend only
- **21 hackathon endpoints** — complete judge/submission/scoring system mapped

---

## 🔥 New Unauthenticated Endpoints Found

### 1. LinkedIn Profile Lookup (NO AUTH!) — High

```bash
GET /v1/directory/retrieve/linkedin/{username}
# Returns: name, country, location, languages, photo, LinkedIn data
```

```json
{
  "detail": "linkedin profile retrieved successfully",
  "data": {
    "username": "xiaojerry",
    "country": "United States",
    "first_name": "Jerry",
    "last_name": "Xiao",
    "languages": ["Cantonese", "Chinese", "English", "Mandrin"],
    "location": "Sugar Land, Texas, United States",
    "photo_url": "https://cdn.cerebralvalley.ai/linked..."
  }
}
```

**Impact:** Anyone's LinkedIn profile data can be retrieved by username. No rate limiting observed.

### 2. Hackathon Gallery Submissions (NO AUTH!) — Medium

```bash
GET /v1/event/{slug}/hackathon/gallery
# Returns: ALL submissions with team names, member user IDs, avatars
```

**Claude Code Hackathon:** 227 submissions returned with full team data.

### 3. Public Vote Check (NO AUTH) — Low

```bash
GET /v1/event/{slug}/hackathon/votes?browserId={id}
# Returns: "Public voting not enabled" or vote data
```

Tracks by `browserId` (client fingerprint), not auth. IP tracking enabled (`hackathonPublicVotingTrackIp: true`).

### 4. Batch LinkedIn Lookup (PARTIAL AUTH?)

```bash
POST /v1/directory/retrieve/linkedins
Body: {"usernames": ["user1", "user2"]}
# Needs correct field name 'usernames' not 'linkedins'
```

---

## Complete API Route Map (63 endpoints)

### Event Management (Auth Required)
| Method | Route | Purpose |
|---|---|---|
| POST | `/event/create` | Create event |
| PATCH | `/event/{id}/update` | Update event |
| DELETE | `/event/{id}/delete` | Delete event |
| GET | `/event/{id}/analytics` | Analytics dashboard |
| GET | `/event/{id}/analytics/people/bulk` | Bulk people analytics |
| GET | `/event/{id}/applicants/csv?salesforceFormat=` | Export applicants |
| GET | `/event/{id}/applicant/{id}` | Single applicant |
| PATCH | `/event/{id}/applicant/{id}/update` | Update applicant status |
| POST | `/event/{id}/blast` | Send email blast |
| GET | `/event/{id}/blasts` | List blasts |
| GET | `/event/{id}/insights` | Event insights |
| GET | `/event/{id}/utm` | UTM tracking |
| POST | `/event/{id}/invite/send` | Send invitation |
| POST | `/event/invite/{id}/answer` | Answer invitation |

### Event (Public / Mixed)
| Method | Route | Auth? | Purpose |
|---|---|---|---|
| GET | `/event/{slug}` | ❌ No | Full event detail + userRoles |
| GET | `/event/{id}/partners` | ❌ No | Partner details |
| POST | `/event/{id}/apply` | ✅ Yes | Apply to event |
| POST | `/event/{id}/unregister` | ✅ Yes | Unregister |
| GET | `/event/{id}/user/application` | ✅ Yes | Check application |
| GET | `/public/event/pull?{status}=true` | ❌ No | Pull events by status |

### Hackathon System (21 endpoints)
| Method | Route | Auth? | Purpose |
|---|---|---|---|
| GET | `/event/{id}/hackathon` | ✅ Yes | Hackathon config |
| PATCH | `/event/{id}/hackathon/update` | ✅ Yes | Update hackathon settings |
| GET | `/event/{id}/hackathon/gallery` | **❌ No** | **All submissions + teams** |
| GET | `/event/{id}/hackathon/gallery/whitelist` | ✅ Yes | Whitelisted submissions |
| GET | `/event/{id}/hackathon/submissions` | ✅ Yes | All submissions (admin) |
| GET | `/event/{id}/hackathon/submissions/user` | ✅ Yes | User's submission |
| GET | `/event/{id}/hackathon/submissions/csv` | ✅ Yes | Export CSV |
| POST | `/event/{id}/hackathon/submit` | ✅ Yes | Submit project |
| PATCH | `/event/{id}/hackathon/submission/{id}/update` | ✅ Yes | Update submission |
| DELETE | `/event/{id}/hackathon/submission/{id}/delete` | ✅ Yes | Delete submission |
| POST | `/event/{id}/hackathon/submission/{id}/vote` | ✅ Yes | Vote on submission |
| GET | `/event/{id}/hackathon/votes?browserId=` | **❌ No** | **Public vote data** |
| GET | `/event/{id}/hackathon/judges` | ✅ Yes | List judges |
| GET | `/event/{id}/hackathon/judge/status` | ✅ Yes | Judge status |
| GET | `/event/{id}/hackathon/judge/submissions` | ✅ Yes | Judge's assigned submissions |
| POST | `/event/{id}/hackathon/judge/submit` | ✅ Yes | Submit score |
| PATCH | `/event/{id}/hackathon/judge/submission/{id}/update` | ✅ Yes | Update score |
| DELETE | `/event/{id}/hackathon/judge/submission/{id}/delete` | ✅ Yes | Delete score |
| POST | `/event/{id}/hackathon/judge/add` | ✅ Yes | Add judge |
| DELETE | `/event/{id}/hackathon/judge/remove` | ✅ Yes | Remove judge |
| GET | `/event/{id}/hackathon/scores?includePending=` | ✅ Yes | Hackathon scores |

### User System
| Method | Route | Auth? | Purpose |
|---|---|---|---|
| GET | `/user/{id}` | ❌ No | User profile |
| GET | `/user/auth/info` | ✅ Yes | Auth details |
| POST | `/user/profile/create` | ✅ Yes | Create profile |
| PATCH | `/user/profile/update` | ✅ Yes | Update profile |
| POST | `/user/migrate/anonymous` | ✅ Yes | Migrate anonymous user |
| GET | `/user/events?` | ✅ Yes | User's events |
| GET | `/user/events/dates?` | ✅ Yes | Event dates |
| GET | `/user/events/hosted/attendees?` | ✅ Yes | Hosted event attendees |
| GET | `/user/events/hosted/attendees/search?` | ✅ Yes | Search attendees |

### Social / Follow System
| Method | Route | Auth? | Purpose |
|---|---|---|---|
| POST | `/user/{id}/follow` | ✅ Yes | Follow user |
| DELETE | `/user/{id}/unfollow` | ✅ Yes | Unfollow |
| GET | `/user/{id}/follow/check` | ✅ Yes | Check follow status |
| GET | `/user/{id}/follow/counts` | ✅ Yes | Follower/following counts |
| GET | `/user/{id}/followers?` | ✅ Yes | List followers |
| GET | `/user/{id}/following?` | ✅ Yes | List following |
| GET | `/user/{id}/followers/search?` | ✅ Yes | Search followers |
| GET | `/user/{id}/following/search?` | ✅ Yes | Search following |
| GET | `/user/{id}/followers/following?` | ✅ Yes | Mutual follows |

### LinkedIn Directory
| Method | Route | Auth? | Purpose |
|---|---|---|---|
| GET | `/directory/retrieve/linkedin/{username}` | **❌ No** | **LinkedIn profile data** |
| POST | `/directory/retrieve/linkedins` | ❓ | Batch LinkedIn lookup |

### Search & Analytics
| Method | Route | Auth? | Purpose |
|---|---|---|---|
| POST | `/search/event/search` | ❌ No | Event search |
| POST | `/search/explain` | ❌ No | Search explanation |
| GET | `/search/users?` | ✅ Yes | User search |
| POST | `/insights/create` | ❌ No | Analytics write |

### Bookmarks
| Method | Route | Auth? | Purpose |
|---|---|---|---|
| GET | `/bookmark/folders?folderType=` | ✅ Yes | List folders |
| POST | `/bookmark/folder/create` | ✅ Yes | Create folder |
| POST | `/bookmark/{id}/item/add` | ✅ Yes | Add bookmark |
| DELETE | `/bookmark/{id}/item/remove` | ✅ Yes | Remove bookmark |
| DELETE | `/bookmark/{id}/delete` | ✅ Yes | Delete folder |
| POST | `/bookmark/{id}/share` | ✅ Yes | Share folder |

### Chat System
| Method | Route | Purpose |
|---|---|---|
| GET | `/chat/{id}?channelType=` | Get chat messages |

### Messaging Microservice (`messaging-api.cerebralvalley.ai`)
| Method | Route | Auth? | Purpose |
|---|---|---|---|
| GET | `/` | ❌ No | "Hello World!" (health check) |
| GET | `/conversations` | ✅ Yes | List conversations |
| GET | `/conversations/{id}` | ✅ Yes | Get conversation |
| DELETE | `/conversations/{id}` | ✅ Yes | Delete conversation |
| GET | `/conversations/inbounds?` | ✅ Yes | Inbound messages |
| GET | `/conversations/inbounds?orgId=` | ✅ Yes | Org inbound messages |
| GET | `/conversations/outbounds?` | ✅ Yes | Outbound messages |
| WS | `wss://{host}/messaging` | ✅ Yes | WebSocket real-time |

### Slack Integration
| Method | Route | Purpose |
|---|---|---|
| POST | `/user/slack/webhook/test` | Test Slack webhook |

---

## Supabase Analysis

**No credentials found.** The only match was `anonicalUrl` (part of `canonicalUrl`). Supabase is fully server-side — no client-side `createClient()` calls, no project URLs, no anon keys.

---

## Architecture Summary

```
cerebralvalley.ai (Next.js/Turbopack SPA)
  ├── api.cerebralvalley.ai/v1 (Express/Node.js)
  │   ├── Event CRUD
  │   ├── Hackathon system (21 endpoints)
  │   ├── User profiles + social
  │   ├── LinkedIn directory
  │   ├── Search (vector + keyword)
  │   └── Bookmarks + Chat
  ├── messaging-api.cerebralvalley.ai (Separate microservice)
  │   ├── REST: conversations CRUD
  │   └── WebSocket: wss://{host}/messaging
  ├── clerk.cerebralvalley.ai (Auth)
  └── Supabase (backend-only, no client access)
```
