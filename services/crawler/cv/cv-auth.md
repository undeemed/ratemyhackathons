# Cerebral Valley — Complete Security Assessment

> Date: 2026-03-13 | Tools: AIDA Exegol, JS reverse engineering, direct HTTP probing
> Type: Authorized penetration test (judge account)
> Scope: Full API surface, auth bypass, privilege escalation

---

## Verdict

**No auth bypass for admin/host access.** Clerk JWT validation is strict — forged tokens, `alg:none`, internal headers, cookie injection all fail. However, **massive unauthenticated data exfiltration** is possible across 12+ public endpoints.

---

## 🔴 Critical: Unauthenticated PII Endpoints

### 1. LinkedIn Profile Scraping (ANY username)

```bash
GET /v1/directory/retrieve/linkedin/{username}
# No auth, no rate limit
```
Returns: full name, country, location, languages, photo, LinkedIn URL, education.

```bash
POST /v1/directory/retrieve/linkedins
Body: {"usernames": ["xiaojerry", "raymond-del-vecchio"]}
# Batch lookup — no auth
```

### 2. User Search & Enumeration

```bash
GET /v1/search/users?query={name}
# No auth — returns userId, firstName, lastName, avatarUrl, handle
```

Tested: `query=admin` → 8 results. `query=ray` → 33 results. `query=ray del vecchio` → found founder accounts.

**Founder accounts discovered:**
| Handle | User ID | Name |
|---|---|---|
| `@ray` | `ea6a9357-2abd-4750-adff-1f82a19fb97f` | Ray Del Vecchio |
| `@raycv` | `user_30xxT2IXYDuq5FIiZdP68BZTLU8` | Ray Del Vecchio |
| `@delvecchio` | `3d9e73ac-406d-4c0b-b13c-c04049e4ba9a` | Ray Del Vecchio |
| `@cv` | `4f83f69e-c8ae-490c-b616-a0b7b9...` | Cerebral Valley |

### 3. Full User Profile Access

```bash
GET /v1/user/{userId}
# No auth — returns full profile
```

Ray Del Vecchio's profile returned:
- GitHub: `raydelvecchio`
- X/Twitter: `raydelvecc`
- LinkedIn: `raymond-del-vecchio`
- Description: "Welcome to the first ever CV profile."
- Avatar, banner, external links, email/phone verification status

### 4. Follow Lists

```bash
GET /v1/user/{userId}/followers?limit=50    # No auth
GET /v1/user/{userId}/follow/counts          # No auth
```

### 5. 227 Hackathon Submissions

```bash
GET /v1/event/{slug}/hackathon/gallery
# No auth — full team data, member IDs, avatars
```

Claude Code Hackathon: 227 submissions exposed. Other events return "Gallery not enabled."

---

## 🟡 Auth Bypass Attempts (All Failed)

| Attack | Result |
|---|---|
| JWT `alg:none` | Server ignores — serves public data only |
| Internal headers (`X-Forwarded-For: 127.0.0.1`) | Blocked |
| Host header injection | nginx 404 |
| Cookie auth (`__session`, `__clerk_db_jwt`) | Rejected |
| Clerk dev browser mode | `instance_type_invalid` (production) |
| Clerk `unsafe_metadata` (role:admin) | CAPTCHA blocks |
| Clerk ticket strategy | `ticket_invalid_code` |
| Organization endpoints | Express 404 (not implemented) |
| Debug/admin/internal endpoints | All Express 404 |
| `.env` file access | 404 |

---

## 🟢 CAPTCHA Bypass (Working)

**Google OAuth bypasses Turnstile CAPTCHA entirely:**

```bash
POST https://clerk.cerebralvalley.ai/v1/client/sign_ins
Body: strategy=oauth_google&redirect_url=https://cerebralvalley.ai
# Returns Google OAuth URL — no CAPTCHA token required
```

Account creation possible without solving any captcha.

---

## Full Unauthenticated Endpoint List (12)

| # | Endpoint | Method | Data |
|---|---|---|---|
| 1 | `/public/event/pull?featured=true` | GET | All featured events |
| 2 | `/public/event/pull?approved=true` | GET | All approved events |
| 3 | `/event/{slug}` | GET | Full event (27 fields, hosts, questions, media, waiver) |
| 4 | `/event/{slug}/partners` | GET | Partner details |
| 5 | `/event/{slug}/hackathon/gallery` | GET | All hackathon submissions + teams |
| 6 | `/event/{slug}/hackathon/votes?browserId=` | GET | Public vote data |
| 7 | `/search/event/search` | POST | Event search (keyword + vector) |
| 8 | `/search/explain` | POST | Search explanation |
| 9 | `/search/users?query=` | GET | **User enumeration — PII** |
| 10 | `/user/{userId}` | GET | **Full user profiles — PII** |
| 11 | `/user/{userId}/followers` | GET | Social graph data |
| 12 | `/user/{userId}/follow/counts` | GET | Follower/following counts |
| 13 | `/directory/retrieve/linkedin/{username}` | GET | **LinkedIn PII — any user** |
| 14 | `/directory/retrieve/linkedins` | POST | **Batch LinkedIn PII** |
| 15 | `/insights/create` | POST | Analytics write (data injection) |

---

## Authenticated Test Results (Judge Account)

| Endpoint | Result |
|---|---|
| Own event + userRoles | ✅ `userRoles: ['judge']` |
| Own profile | ✅ Full profile returned |
| IDOR: other event applicants | 🔴 403 |
| IDOR: update other event | 🔴 403 |
| IDOR: judge other hackathon | 🔴 403 |
| Own hackathon submissions | 🔴 403 (judge can't access via REST) |
| Bookmarks | 🔴 403 |

---

## Architecture

```
                      ┌─ cerebralvalley.ai (Next.js/Turbopack)
                      │
┌─────────────────────┼──────────────────────────┐
│                     │                           │
▼                     ▼                           ▼
api.cerebralvalley.ai  messaging-api.cerebralvalley.ai  clerk.cerebralvalley.ai
(Express + nginx)      (NestJS, "Hello World!")         (Clerk Production)
53 REST endpoints      7 REST + 1 WebSocket              OAuth, JWT, OTP
                       wss://{host}/messaging
                      │
                      ▼
                  Supabase (backend only, no client access)
```

---

## 63 Unique API Endpoints Mapped

Full route map in [cv-reversing.md](./cv-reversing.md).

---

## Information Disclosure Summary

| Finding | Severity | Data |
|---|---|---|
| LinkedIn PII scraping (any username) | **HIGH** | Name, location, education, languages, photo |
| User search + enumeration | **HIGH** | IDs, names, handles of all users |
| Full user profiles without auth | **HIGH** | Social links, email/phone verify status |
| Hackathon submissions without auth | MEDIUM | Team names, member IDs, 227 submissions |
| Event admin config without auth | MEDIUM | 27 fields including hosts, questions, media, waiver |
| Follow lists without auth | MEDIUM | Social graph |
| Google OAuth CAPTCHA bypass | MEDIUM | Account creation possible |
| Analytics injection | MEDIUM | Arbitrary data write |
| User enumeration via Clerk | LOW | Email existence check |
| Internal model names in errors | LOW | `platformEvent`, `userProfile` |
| Zod schema leak | LOW | Validation shapes |
| Easter egg with staff email | INFO | `ray@cv.inc` |
| Turnstile sitekeys | INFO | Smart + invisible keys |
| nginx version disclosure | INFO | Reverse proxy |
