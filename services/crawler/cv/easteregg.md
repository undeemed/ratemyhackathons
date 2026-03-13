# Cerebral Valley API Easter Egg 🥚

> Discovered: 2026-03-13 | Tool: AIDA Exegol + brute-force probing

## The Hidden Message

The CV API at `api.cerebralvalley.ai/v1/event/{invalid-slug}` returns a `curiosity` field
alongside the 404 error detail. The message is split into 3 parts, each with a **12.5% probability**
of appearing on any given request.

### Full Message (Reassembled)

> **"We value engineering curiosity at Cerebral Valley. Send an email to ray@cv.inc with the subject line 'Platform Eatser Eg' for hiring inquiries."**

Note the intentional misspelling: **"Eatser Eg"** instead of "Easter Egg" — a test to see if you actually read it.

### Raw Parts

| Part | Probability | Content |
|------|-------------|---------|
| 1/3  | 12.5%       | `We value engineering curiosity at Cerebral Valley. Send an email to...` |
| 2/3  | 12.5%       | `...ray@cv.inc with the subject line...` |
| 3/3  | 12.5%       | `...'Platform Eatser Eg' for hiring inquiries.` |

### How to Trigger

```bash
curl https://api.cerebralvalley.ai/v1/event/anything-here
```

**Response (when curiosity appears):**
```json
{
  "detail": "Event with slug anything-here not found",
  "curiosity": {
    "message": "We value engineering curiosity at Cerebral Valley. Send an email to...",
    "metadata": {
      "messagePart": "1/3",
      "probability": 0.125
    }
  }
}
```

### Discovery Method

Sent 100 requests with random slugs (`probe-0` through `probe-99`) to `GET /v1/event/{slug}`.
Collected unique `messagePart` values. All 3 parts were found within the first 16 requests.

### What It Tells Us

- **ray@cv.inc** — Likely Ray, an engineer or hiring manager at Cerebral Valley
- **cv.inc** — The company's internal email domain (not `.ai`)
- The misspelling is deliberate — proves the applicant actually found and read the easter egg
- 12.5% probability = ~1 in 8 chance per request, ensuring only persistent engineers find all parts
