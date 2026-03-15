# RateMyProfessors-Style UI/UX Flows for RateMyHackathons

**Date**: 2026-03-14
**Status**: Approved

## Overview

Replicate RateMyProfessors' user flow for RateMyHackathons. Users search by company or event name, view multi-dimensional ratings, submit reviews with per-category scores, and compare entities side-by-side.

### Entity Mapping

| RMP | RMH | Role |
|-----|-----|------|
| School | Company | Aggregate multi-dimensional ratings |
| Professor | Event | Individual reviews with category breakdowns |
| School Rating | Company Review | 10 category scores + review text |
| Professor Rating | Event Review | 10 category scores + review text |

Both companies and events receive their own multi-dimensional ratings and reviews.

## Architecture

### Components

| Component | Stack | Port |
|-----------|-------|------|
| Backend | Rust/Actix-Web | `:8080` |
| Frontend | SvelteKit + svelte-clerk + Tailwind v4 | `:5173` |
| Crawler | Python (Scrapling) | N/A |
| Analytics | Rust + SvelteKit | `:8081`/`:5174` |
| Admin Dashboard (Phase 4) | SvelteKit | `:5175` |

### Auth

- **Frontend**: `svelte-clerk` package for sign-in/sign-up UI, SSR via `withClerkHandler()` hooks
- **Backend**: Clerk JWT verification via `jsonwebtoken` crate against Clerk's JWKS endpoint (RS256). No Clerk SDK on the Rust side.
- **Flow**: Browse freely without auth. Submitting reviews requires Clerk sign-in. JWT `sub` claim maps to `users.clerk_id`.

### Design System

Keep the existing editorial brutalist B&W aesthetic. Color only appears in score badges:

- **Green** `#4caf50`: 4.0-5.0 (great)
- **Yellow** `#ffc107`: 3.0-3.9 (okay)
- **Red** `#ef5350`: 1.0-2.9 (poor)

## Build Phases

| Phase | Scope | Deliverables |
|-------|-------|-------------|
| 1 | Schema + API | Migration, enhanced search/detail/review endpoints, tags |
| 2 | Clerk Auth | svelte-clerk setup, JWT middleware in Rust, user sync |
| 3 | Frontend Rebuild | All 7 pages, color scores, autocomplete, rate forms |
| 4 | Advanced | Compare tool, admin dashboard, tag voting, distribution charts |

Doc updates (CLAUDE.md, README.md, TODO.md, CHANGELOG.md) are deliverables in every phase.

---

## Phase 1: Schema + API

### Rating Categories

10 hackathon-specific categories, each scored 1-5:

| Category | Key | Measures |
|----------|-----|----------|
| Organization | `organization` | Schedule, communication, logistics |
| Prize Quality | `prizes` | Prize pool value, fairness |
| Mentorship | `mentorship` | Mentor availability, helpfulness |
| Judging | `judging` | Fairness, expertise, feedback quality |
| Venue | `venue` | Space, power, seating, accessibility |
| Food | `food` | Quality, dietary options, availability |
| Swag | `swag` | Quality and quantity of swag |
| Networking | `networking` | Sponsor interaction, peer connections |
| Communication | `communication` | Pre/during/post event comms |
| Vibes | `vibes` | Overall atmosphere, energy, inclusivity |

### Schema Migration

```sql
-- Per-category scores stored alongside each review
CREATE TABLE review_ratings (
    review_id   UUID NOT NULL REFERENCES reviews(id) ON DELETE CASCADE,
    category    TEXT NOT NULL,
    score       SMALLINT NOT NULL CHECK (score BETWEEN 1 AND 5),
    PRIMARY KEY (review_id, category)
);

-- Reviews can now target a company OR event
ALTER TABLE reviews
    ADD COLUMN company_id UUID REFERENCES companies(id),
    ADD COLUMN would_return BOOLEAN,
    ALTER COLUMN event_id DROP NOT NULL;

ALTER TABLE reviews ADD CHECK (
    (event_id IS NOT NULL AND company_id IS NULL) OR
    (event_id IS NULL AND company_id IS NOT NULL)
);

-- Crowd-sourced tags
CREATE TABLE tags (
    id   UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE review_tags (
    review_id UUID NOT NULL REFERENCES reviews(id) ON DELETE CASCADE,
    tag_id    UUID NOT NULL REFERENCES tags(id),
    PRIMARY KEY (review_id, tag_id)
);

-- Clerk user sync
ALTER TABLE users
    ADD COLUMN clerk_id TEXT UNIQUE,
    ADD COLUMN avatar_url TEXT;

-- Indexes
CREATE INDEX idx_review_ratings_review ON review_ratings (review_id);
CREATE INDEX idx_review_ratings_category ON review_ratings (category);
CREATE INDEX idx_review_tags_review ON review_tags (review_id);
CREATE INDEX idx_review_tags_tag ON review_tags (tag_id);
CREATE INDEX idx_reviews_company ON reviews (company_id);
CREATE INDEX idx_users_clerk ON users (clerk_id);
```

### API Changes

#### Search (enhanced)

```
GET /api/search?q=...&type=event|company
```

Response adds `avg_rating`, `review_count`, `would_return_pct` per result for color-coded score badges in the UI.

#### Company detail (enhanced)

```
GET /api/companies/{id}
```

Returns:
- `avg_rating: f64`, `review_count: i64`, `would_return_pct: f64`
- `category_ratings: [{category: "organization", avg: 4.2}, ...]`
- `top_tags: [{name: "well-organized", count: 12}, ...]`
- `rating_distribution: {5: 20, 4: 15, 3: 5, 2: 2, 1: 1}`
- `reviews[]` with per-category scores, tags, votes

Key query for category averages:
```sql
SELECT rr.category, AVG(rr.score)::float8 as avg
FROM review_ratings rr
JOIN reviews r ON r.id = rr.review_id
WHERE r.company_id = $1
GROUP BY rr.category
```

#### Event detail (enhanced)

```
GET /api/events/{id}
```

Same additions as company: `category_ratings`, `top_tags`, `rating_distribution`, `would_return_pct`.

#### Submit review (updated)

```
POST /api/reviews  [Auth required]
```

```json
{
  "event_id": "uuid | null",
  "company_id": "uuid | null",
  "rating": 4,
  "title": "Great hackathon",
  "body": "...",
  "would_return": true,
  "category_ratings": {
    "organization": 5, "prizes": 4, "mentorship": 3,
    "judging": 4, "venue": 5, "food": 3,
    "swag": 2, "networking": 4, "communication": 5, "vibes": 5
  },
  "tag_ids": ["uuid", "uuid"]
}
```

The `rating` field is computed as the average of the 10 category scores (rounded to nearest integer) at insert time. All 10 categories are required. Inserts into `reviews` + batch-inserts 10 rows into `review_ratings` + inserts into `review_tags`. All in a transaction.

#### Tags

```
GET  /api/tags                                          — list all tags
GET  /api/tags/top?entity_type=event&entity_id=uuid     — top tags for entity
POST /api/tags  [Auth required]                         — create a new tag
```

`POST /api/tags` body: `{"name": "well-organized"}`. Returns existing tag if name matches (case-insensitive). Used by the rate form's "suggest new tag" feature.

#### Compare

```
GET /api/compare?type=company&ids=uuid1,uuid2
GET /api/compare?type=event&ids=uuid1,uuid2
```

Returns side-by-side `category_ratings`, `avg_rating`, `review_count`, `would_return_pct` for each entity.

---

## Phase 2: Clerk Auth

### Frontend (svelte-clerk)

```bash
cd frontend && bun add svelte-clerk
```

**hooks.server.ts**:
```ts
import { withClerkHandler } from 'svelte-clerk/server';
export const handle = withClerkHandler();
```

**+layout.server.ts**:
```ts
import { buildClerkProps } from 'svelte-clerk/server';
export const load = ({ locals }) => ({
  ...buildClerkProps(locals.auth())
});
```

**+layout.svelte**: Wrap app in `<ClerkProvider>`.

**Auth pages**: `/sign-in` and `/sign-up` using Clerk's `<SignIn />` and `<SignUp />` components, styled to match brutalist theme.

### Backend (Rust JWT middleware)

Actix-Web middleware that:
1. Extracts `Authorization: Bearer <token>` header
2. Fetches Clerk JWKS from `https://<clerk-domain>/.well-known/jwks.json` (cached)
3. Verifies JWT signature (RS256) using `jsonwebtoken` crate
4. Extracts `sub` claim (Clerk user ID)
5. Looks up or creates user in `users` table by `clerk_id`
6. Attaches `user_id: Uuid` to request extensions

Applied only to mutating endpoints (POST/PUT/DELETE). GET remains public.

### Environment Variables

**Frontend** (`frontend/.env`):
- `PUBLIC_CLERK_PUBLISHABLE_KEY`
- `CLERK_SECRET_KEY`

**Backend** (`backend/.env`):
- `CLERK_JWKS_URL` (or `CLERK_PEM_PUBLIC_KEY`)
- `CLERK_ISSUER`

---

## Phase 3: Frontend Rebuild

### Color Scoring

```ts
function scoreColor(score: number): string {
  if (score >= 4.0) return '#4caf50'; // green
  if (score >= 3.0) return '#ffc107'; // yellow
  return '#ef5350';                    // red
}
```

### Pages

#### 1. Homepage (`/`)

- Keep globe + grain + hero scroll sections
- Replace search with **autocomplete search bar** (companies + events combined dropdown)
- Secondary links: "Browse all events" / "Browse all companies"

#### 2. Search Results (`/search?q=...`)

- Heading: "12 results for 'devpost'"
- Tabs: Events | Companies
- Result cards (RMP-style):
  - Left: `QUALITY` label + color-coded score square + `X reviews`
  - Right: Name (bold) + location/date + "would return" %
  - Entire card is a clickable link
- "Show More" pagination
- Empty: "Don't see it? Add an Event / Add a Company"

#### 3. Company Detail (`/companies/{id}`)

- Header: Name, description, "View all Events" link
- CTAs: "Rate" → `/companies/{id}/rate`, "Compare"
- Giant overall score (color-coded) + "Overall Quality"
- 10 category scores in 2-column grid with icons
- "X% would return" metric
- Top Tags as pill badges
- Rating Distribution: Awesome/Great/Good/OK/Awful bar chart
- Reviews list with per-category bars, thumbs, flags

#### 4. Event Detail (`/events/{id}`)

- Same structure as company but with event metadata (dates, location, website, sponsors)
- Same rating summary, categories, tags, distribution, reviews
- Companies section: clickable sponsor/organizer pills

#### 5. Rate Form (`/companies/{id}/rate`, `/events/{id}/rate`)

- Auth gate: Clerk `<SignIn />` if not authenticated
- 10 category sliders (1-5, Awful → Awesome)
- "Would attend again?" Yes/No toggle
- Tags: multi-select from existing + suggest new
- Review text: 350 char min, 5000 char max, guidelines sidebar
- Submit: disabled until all categories rated

#### 6. Compare (`/compare?type=company&ids=uuid1,uuid2`)

- Side-by-side: pre-filled entity vs. search for second
- Category rows with both scores + color indicators
- Reset + Share buttons

#### 7. Auth Pages (`/sign-in`, `/sign-up`)

- Clerk components styled to brutalist theme (black bg, white text, no border-radius)

### New Components

- `ScoreBadge.svelte` — color-coded score square (green/yellow/red)
- `CategoryGrid.svelte` — 2-column grid of category scores with icons
- `RatingDistribution.svelte` — horizontal bar chart (Awesome→Awful)
- `TagPills.svelte` — row of tag badges
- `SearchAutocomplete.svelte` — homepage autocomplete dropdown
- `RatingSlider.svelte` — 1-5 slider with labels
- `CompareLayout.svelte` — side-by-side comparison container

---

## Phase 4: Advanced Features

### Compare Tool

Full implementation of `/compare` page with search-to-add second entity, shareable URLs, and visual bar comparison.

### Admin Dashboard (`services/admin/`)

Separate SvelteKit app on `:5175`, Clerk-protected with role-based access.

**Pages:**
- Overview: totals for events, companies, reviews, users, flagged items
- Events CRUD: list/search/edit/delete, merge duplicates, toggle visibility
- Companies CRUD: same pattern
- Reviews moderation: flagged queue, approve/reject/remove
- Users: list, ban/suspend, review history
- Crawler status: `scrape_sources` table, enable/disable spiders, last crawl, manual trigger
- Tags management: create/rename/merge/delete

### Tag Voting

Users can upvote existing tags on a review or suggest new ones. Top 5 tags shown on entity detail pages.

### Rating Distribution Charts

Horizontal bar chart showing count of reviews at each rating level (5=Awesome, 4=Great, 3=Good, 2=OK, 1=Awful), color-coded.

### "Would Return" Metric

Percentage displayed as `X% would attend again` on entity cards and detail pages.

---

## Key Queries Reference

**Category averages for an entity:**
```sql
SELECT rr.category, AVG(rr.score)::float8 as avg
FROM review_ratings rr
JOIN reviews r ON r.id = rr.review_id
WHERE r.company_id = $1  -- or r.event_id = $1
GROUP BY rr.category
```

**Top tags for an entity:**
```sql
SELECT t.name, COUNT(*) as count
FROM review_tags rt
JOIN tags t ON t.id = rt.tag_id
JOIN reviews r ON r.id = rt.review_id
WHERE r.company_id = $1  -- or r.event_id = $1
GROUP BY t.name
ORDER BY count DESC
LIMIT 5
```

**Rating distribution:**
```sql
SELECT rating, COUNT(*) as count
FROM reviews
WHERE company_id = $1  -- or event_id = $1
GROUP BY rating
ORDER BY rating DESC
```

**Would return percentage:**
```sql
SELECT
  COUNT(*) FILTER (WHERE would_return = true) * 100.0 / NULLIF(COUNT(*), 0) as pct
FROM reviews
WHERE company_id = $1  -- or event_id = $1
```

**Enhanced search with scores:**
```sql
SELECT e.id, e.name,
       ts_rank(e.search_vector, plainto_tsquery('english', $1)) as rank,
       COALESCE(AVG(r.rating)::float8, NULL) as avg_rating,
       COUNT(r.id) as review_count
FROM events e
LEFT JOIN reviews r ON r.event_id = e.id
WHERE e.search_vector @@ plainto_tsquery('english', $1)
GROUP BY e.id
ORDER BY rank DESC
LIMIT $2
```
