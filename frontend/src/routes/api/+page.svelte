<script lang="ts">
	import { fadeIn, staggerChildren } from '$lib/animations/gsap';

	type Endpoint = {
		method: string;
		path: string;
		desc: string;
		auth?: boolean;
		params?: string;
		body?: string;
		response?: string;
	};

	type Section = {
		name: string;
		desc: string;
		endpoints: Endpoint[];
	};

	let expandedIdx: number | null = $state(null);

	function toggle(i: number) {
		expandedIdx = expandedIdx === i ? null : i;
	}

	const baseUrl = 'https://ratemyhackathons.com/api';

	const sections: Section[] = [
		{
			name: 'Events',
			desc: 'Hackathon event listings with ratings, reviews, and company associations.',
			endpoints: [
				{
					method: 'GET',
					path: '/api/events',
					desc: 'List events (paginated, filterable)',
					params: '?page=1&per_page=20&company_id=uuid',
					response: `{
  "data": [{
    "id": "uuid", "name": "HackMIT 2025",
    "location": "Cambridge, MA",
    "start_date": "2025-10-01",
    "avg_rating": 4.2, "review_count": 15,
    "companies": [{ "id": "uuid", "name": "Google", "role": "sponsor" }]
  }],
  "total": 100, "page": 1, "per_page": 20
}`,
				},
				{
					method: 'GET',
					path: '/api/events/{id}',
					desc: 'Event detail with companies, reviews, category ratings, and tags',
					response: `{
  "id": "uuid", "name": "HackMIT 2025",
  "location": "Cambridge, MA",
  "start_date": "2025-10-01", "end_date": "2025-10-02",
  "avg_rating": 4.2, "review_count": 15,
  "category_ratings": [{ "category": "vibes", "avg": 4.8 }],
  "top_tags": [{ "id": "uuid", "name": "beginner-friendly", "count": 12 }],
  "rating_distribution": { "1": 2, "2": 1, "3": 3, "4": 5, "5": 4 },
  "companies": [...], "reviews": [...]
}`,
				},
				{
					method: 'POST',
					path: '/api/events',
					desc: 'Create event',
					auth: true,
					body: `{
  "name": "HackMIT 2025",
  "description": "Annual hackathon",
  "location": "Cambridge, MA",
  "url": "https://hackmit.org",
  "start_date": "2025-10-01",
  "end_date": "2025-10-02",
  "company_ids": ["uuid"]
}`,
					response: `// 201 Created — full event object`,
				},
				{
					method: 'GET',
					path: '/api/events/globe',
					desc: 'Globe markers — events with lat/lng for WebGL globe',
					response: `[{
  "id": "uuid", "name": "HackMIT",
  "latitude": 42.3601, "longitude": -71.0589,
  "start_date": "2025-10-01"
}]`,
				},
			],
		},
		{
			name: 'Companies',
			desc: 'Hackathon sponsors and organizers.',
			endpoints: [
				{
					method: 'GET',
					path: '/api/companies',
					desc: 'List companies (paginated)',
					params: '?page=1&per_page=20',
					response: `{
  "data": [{
    "id": "uuid", "name": "Google",
    "logo_url": "https://...",
    "website": "https://google.com",
    "event_count": 12
  }],
  "total": 50, "page": 1, "per_page": 20
}`,
				},
				{
					method: 'GET',
					path: '/api/companies/{id}',
					desc: 'Company detail with events, category ratings, and tags',
					response: `{
  "id": "uuid", "name": "Google",
  "website": "https://google.com",
  "events": [{ "id": "uuid", "name": "HackMIT", "role": "sponsor" }],
  "category_ratings": [...], "top_tags": [...]
}`,
				},
				{
					method: 'POST',
					path: '/api/companies',
					desc: 'Create company',
					auth: true,
					body: `{
  "name": "Google",
  "logo_url": "https://...",
  "website": "https://google.com",
  "description": "..."
}`,
				},
			],
		},
		{
			name: 'Users',
			desc: 'User profiles with social links and review history.',
			endpoints: [
				{
					method: 'GET',
					path: '/api/users',
					desc: 'List users (paginated)',
					params: '?page=1&per_page=20',
					response: `{
  "data": [{
    "id": "uuid", "username": "alice",
    "display_name": "Alice Chen",
    "review_count": 5
  }],
  "total": 200, "page": 1, "per_page": 20
}`,
				},
				{
					method: 'GET',
					path: '/api/users/{id}',
					desc: 'User profile with reviews and social links',
					response: `{
  "id": "uuid", "username": "alice",
  "display_name": "Alice Chen", "bio": "...",
  "socials": {
    "github": "alicechen", "twitter": "alice_dev",
    "linkedin": "alice-chen", "website": "https://alice.dev"
  },
  "reviews": [...]
}`,
				},
				{
					method: 'POST',
					path: '/api/users',
					desc: 'Create user',
					auth: true,
					body: `{
  "username": "alice",
  "email": "alice@example.com",
  "display_name": "Alice Chen",
  "bio": "Full-stack developer",
  "github": "alicechen"
}`,
				},
			],
		},
		{
			name: 'Reviews',
			desc: 'Multi-category reviews with voting and threaded comments.',
			endpoints: [
				{
					method: 'POST',
					path: '/api/users/{user_id}/reviews',
					desc: 'Create review with 10 category scores',
					auth: true,
					body: `{
  "event_id": "uuid",
  "title": "Amazing hackathon!",
  "body": "Detailed review text (350-5000 chars)...",
  "would_return": true,
  "category_ratings": {
    "organization": 5, "prizes": 4,
    "mentorship": 5, "judging": 4,
    "venue": 3, "food": 4, "swag": 3,
    "networking": 5, "communication": 4, "vibes": 5
  },
  "tag_ids": ["uuid"]
}`,
					response: `// 201 Created — full review with computed overall rating`,
				},
				{
					method: 'GET',
					path: '/api/reviews/{id}',
					desc: 'Review detail with votes and threaded comments',
					response: `{
  "id": "uuid", "rating": 5,
  "title": "Amazing!", "body": "...",
  "votes": { "helpful": 12, "unhelpful": 3 },
  "comments": [{
    "id": "uuid", "username": "bob",
    "body": "Great review!",
    "replies": [{ "username": "alice", "body": "Thanks!", "replies": [] }]
  }]
}`,
				},
				{
					method: 'POST',
					path: '/api/reviews/{id}/vote',
					desc: 'Vote helpful/unhelpful (upsert)',
					auth: true,
					body: `{ "user_id": "uuid", "helpful": true }`,
				},
				{
					method: 'POST',
					path: '/api/reviews/{id}/comments',
					desc: 'Add comment or threaded reply',
					auth: true,
					body: `{
  "user_id": "uuid",
  "body": "Great review!",
  "parent_comment_id": "uuid"  // optional — omit for top-level
}`,
				},
				{
					method: 'GET',
					path: '/api/reviews/{id}/comments',
					desc: 'Get threaded comment tree',
					response: `[{
  "id": "uuid", "username": "bob",
  "body": "Great review!",
  "replies": [{ "username": "alice", "body": "Thanks!", "replies": [] }]
}]`,
				},
			],
		},
		{
			name: 'Search',
			desc: 'Full-text search across events, companies, and users.',
			endpoints: [
				{
					method: 'GET',
					path: '/api/search',
					desc: 'Full-text search with type filtering',
					params: '?q=hackathon&type=event|company|user&per_page=20',
					response: `{
  "events": [{ "id": "uuid", "name": "HackMIT", "rank": 0.95, "avg_rating": 4.2, "review_count": 15 }],
  "companies": [{ "id": "uuid", "name": "Google", "rank": 0.87 }],
  "users": [{ "id": "uuid", "name": "hackfan", "rank": 0.72 }],
  "total": 25
}`,
				},
			],
		},
		{
			name: 'Tags',
			desc: 'Crowd-sourced labels for events and companies.',
			endpoints: [
				{
					method: 'GET',
					path: '/api/tags',
					desc: 'List all tags',
					response: `[{ "id": "uuid", "name": "beginner-friendly", "count": 42 }]`,
				},
				{
					method: 'GET',
					path: '/api/tags/top',
					desc: 'Top 5 tags for an entity',
					params: '?entity_type=event&entity_id=uuid',
					response: `[{ "id": "uuid", "name": "beginner-friendly", "count": 12 }]`,
				},
				{
					method: 'POST',
					path: '/api/tags',
					desc: 'Create tag (returns existing if name matches)',
					auth: true,
					body: `{ "name": "beginner-friendly" }`,
				},
			],
		},
		{
			name: 'Compare',
			desc: 'Side-by-side entity comparison.',
			endpoints: [
				{
					method: 'GET',
					path: '/api/compare',
					desc: 'Compare entities side-by-side with category breakdowns',
					params: '?type=event&ids=uuid1,uuid2',
					response: `[{
  "id": "uuid", "name": "HackMIT",
  "avg_rating": 4.2, "review_count": 15,
  "would_return_pct": 87.5,
  "category_ratings": [{ "category": "vibes", "avg": 4.8 }],
  "top_tags": [...]
}]`,
				},
			],
		},
	];

	const methodColor: Record<string, string> = {
		GET: 'text-[#4caf50]',
		POST: 'text-[#ffc107]',
		PUT: 'text-[#2196f3]',
		DELETE: 'text-[#ef5350]',
	};


</script>

<svelte:head>
	<title>API — RateMyHackathons</title>
</svelte:head>

<div class="mx-auto max-w-[1400px] px-6 py-24">
	<!-- Header -->
	<div class="mb-20" use:fadeIn>
		<a
			href="/"
			class="mb-6 inline-block text-xs uppercase tracking-[0.2em] text-dim transition-colors hover:text-text"
			>&larr; Back</a
		>
		<h1 class="mt-2 font-display text-6xl italic sm:text-7xl lg:text-8xl">API</h1>
		<p class="mt-4 max-w-2xl text-sm leading-relaxed text-muted">
			REST API for hackathon events, companies, users, and reviews. All list endpoints use
			correlated subqueries — one SQL query, zero N+1 problems.
		</p>
		<div class="mt-6 h-px w-24 bg-border"></div>
	</div>

	<!-- Overview -->
	<section class="mb-16" use:fadeIn>
		<div class="grid grid-cols-1 gap-8 lg:grid-cols-3">
			<div class="border border-border bg-surface p-6">
				<span class="text-[10px] uppercase tracking-[0.3em] text-dim">Base URL</span>
				<p class="mt-2 font-mono text-sm text-text break-all">{baseUrl}</p>
			</div>
			<div class="border border-border bg-surface p-6">
				<span class="text-[10px] uppercase tracking-[0.3em] text-dim">Auth</span>
				<p class="mt-2 text-xs text-muted">
					Clerk JWT in <code class="text-text">Authorization: Bearer &lt;token&gt;</code> header.
					Required for POST endpoints.
				</p>
			</div>
			<div class="border border-border bg-surface p-6">
				<span class="text-[10px] uppercase tracking-[0.3em] text-dim">Format</span>
				<p class="mt-2 text-xs text-muted">
					JSON request/response. IDs are UUIDv7. Timestamps are ISO 8601.
					Input sanitized with <code class="text-text">ammonia</code>.
				</p>
			</div>
		</div>
	</section>

	<!-- Quick Reference -->
	<section class="mb-16" use:fadeIn>
		<span class="text-[10px] uppercase tracking-[0.3em] text-dim">Quick Reference</span>
		<h2 class="mt-3 mb-8 font-display text-3xl italic">All Endpoints</h2>
		<div class="border border-border">
			<div class="flex items-center border-b border-border bg-surface px-4 py-2">
				<span class="w-16 shrink-0 text-[10px] uppercase tracking-[0.3em] text-dim">Method</span>
				<span class="flex-1 pl-4 text-[10px] uppercase tracking-[0.3em] text-dim">Path</span>
				<span class="hidden w-64 shrink-0 text-right text-[10px] uppercase tracking-[0.3em] text-dim sm:block">Description</span>
			</div>
			{#each sections as section}
				{#each section.endpoints as ep}
					<div class="flex items-center border-b border-border px-4 py-3 last:border-b-0">
						<span class="w-16 shrink-0 font-mono text-[11px] font-bold {methodColor[ep.method] ?? 'text-muted'}">{ep.method}</span>
						<span class="flex-1 pl-4 font-mono text-xs text-text">{ep.path}</span>
						<span class="hidden w-64 shrink-0 text-right text-xs text-muted sm:block">{ep.desc}</span>
					</div>
				{/each}
			{/each}
		</div>
	</section>

	<!-- Detailed Sections -->
	{#each sections as section, si (section.name)}
		<section class="mb-20" use:fadeIn>
			<div class="mb-2 flex items-baseline gap-4">
				<span class="text-[10px] uppercase tracking-[0.3em] text-dim">{String(si + 1).padStart(2, '0')}</span>
				<h2 class="font-display text-4xl italic">{section.name}</h2>
			</div>
			<p class="mb-8 text-xs text-muted">{section.desc}</p>

			<div class="flex flex-col gap-px border border-border">
				{#each section.endpoints as ep, ei}
					{@const idx = si * 100 + ei}
					<div class="bg-surface">
						<button
							type="button"
							class="flex w-full cursor-pointer items-center gap-4 px-6 py-4 text-left transition-colors hover:bg-elevated"
							onclick={() => toggle(idx)}
						>
							<span class="w-14 shrink-0 font-mono text-xs font-bold {methodColor[ep.method] ?? 'text-muted'}">{ep.method}</span>
							<span class="flex-1 font-mono text-sm text-text">{ep.path}</span>
							{#if ep.auth}
								<span class="border border-border px-2 py-0.5 text-[9px] uppercase tracking-[0.2em] text-dim">Auth</span>
							{/if}
							<span class="text-dim transition-transform" class:rotate-180={expandedIdx === idx}>&#x25BE;</span>
						</button>

						{#if expandedIdx === idx}
							<div class="border-t border-border px-6 py-6">
								<p class="mb-4 text-xs text-muted">{ep.desc}</p>

								{#if ep.params}
									<div class="mb-4">
										<span class="text-[10px] uppercase tracking-[0.3em] text-dim">Query Params</span>
										<pre class="mt-2 overflow-x-auto bg-bg p-4 font-mono text-xs text-accent">{ep.params}</pre>
									</div>
								{/if}

								{#if ep.body}
									<div class="mb-4">
										<span class="text-[10px] uppercase tracking-[0.3em] text-dim">Request Body</span>
										<pre class="mt-2 overflow-x-auto bg-bg p-4 font-mono text-xs text-accent">{ep.body}</pre>
									</div>
								{/if}

								{#if ep.response}
									<div>
										<span class="text-[10px] uppercase tracking-[0.3em] text-dim">Response</span>
										<pre class="mt-2 overflow-x-auto bg-bg p-4 font-mono text-xs text-accent">{ep.response}</pre>
									</div>
								{/if}
							</div>
						{/if}
					</div>
				{/each}
			</div>
		</section>
	{/each}

	<!-- Architecture Note -->
	<section class="border-t border-border pt-16" use:fadeIn>
		<div class="grid grid-cols-1 gap-12 lg:grid-cols-2">
			<div>
				<span class="text-[10px] uppercase tracking-[0.3em] text-dim">Architecture</span>
				<h2 class="mt-3 font-display text-4xl italic">N+1 free</h2>
				<p class="mt-4 text-xs leading-relaxed text-muted">
					All list endpoints use correlated subqueries instead of N+1 loops. One SQL query
					regardless of result count. Input is validated with <code class="text-text">validator</code>
					and sanitized with <code class="text-text">ammonia</code> before database insertion.
				</p>
			</div>
			<div class="border border-border bg-surface p-6">
				<span class="text-[10px] uppercase tracking-[0.3em] text-dim">Example — List Companies</span>
				<pre class="mt-4 overflow-x-auto font-mono text-xs leading-relaxed text-accent">SELECT c.id, c.name, c.logo_url,
       (SELECT COUNT(*)
        FROM event_companies
        WHERE company_id = c.id
       ) as event_count
FROM companies c
ORDER BY c.name ASC
LIMIT $1 OFFSET $2</pre>
			</div>
		</div>
	</section>
</div>
