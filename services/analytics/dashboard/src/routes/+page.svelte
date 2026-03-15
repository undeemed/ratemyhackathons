<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import {
		fetchCrawlStats,
		fetchCrawlSources,
		fetchRatingDistribution,
		fetchRecentReviews,
		fetchTrending,
		fetchEvents,
		fetchCompanies,
		fetchUsers,
		connectLiveFeed
	} from '$lib/api';

	let stats = $state<any>(null);
	let sources = $state<any[]>([]);
	let ratings = $state<any[]>([]);
	let reviews = $state<any[]>([]);
	let trending = $state<any[]>([]);
	let liveEvents = $state<any[]>([]);
	let liveFeed: EventSource | null = $state(null);

	// Main API counts
	let eventTotal = $state(0);
	let companyTotal = $state(0);
	let userTotal = $state(0);

	onMount(async () => {
		const results = await Promise.allSettled([
			fetchCrawlStats(),
			fetchCrawlSources(),
			fetchRatingDistribution(),
			fetchRecentReviews(5),
			fetchTrending(30, 5),
			fetchEvents(1, 1),
			fetchCompanies(1, 1),
			fetchUsers(1, 1),
		]);

		if (results[0].status === 'fulfilled') stats = results[0].value;
		if (results[1].status === 'fulfilled') sources = results[1].value;
		if (results[2].status === 'fulfilled') ratings = results[2].value;
		if (results[3].status === 'fulfilled') reviews = results[3].value;
		if (results[4].status === 'fulfilled') trending = results[4].value;
		if (results[5].status === 'fulfilled') eventTotal = results[5].value?.total ?? 0;
		if (results[6].status === 'fulfilled') companyTotal = results[6].value?.total ?? 0;
		if (results[7].status === 'fulfilled') userTotal = results[7].value?.total ?? 0;

		liveFeed = connectLiveFeed((event) => {
			liveEvents = [{ ...event, ts: new Date().toLocaleTimeString() }, ...liveEvents.slice(0, 49)];
		});
	});

	onDestroy(() => {
		liveFeed?.close();
	});

	function timeAgo(dateStr: string): string {
		const diff = Date.now() - new Date(dateStr).getTime();
		const mins = Math.floor(diff / 60000);
		if (mins < 1) return 'just now';
		if (mins < 60) return `${mins}m ago`;
		const hrs = Math.floor(mins / 60);
		if (hrs < 24) return `${hrs}h ago`;
		return `${Math.floor(hrs / 24)}d ago`;
	}
</script>

<h1 class="mb-6 text-xl font-semibold text-text">Overview</h1>

<!-- Top Stats -->
<div class="mb-6 grid grid-cols-2 gap-4 lg:grid-cols-4">
	<div class="rounded-xl border border-border bg-surface p-5">
		<p class="mb-1 text-xs uppercase tracking-wider text-text-muted">Total Events</p>
		<p class="font-mono text-3xl font-bold text-text">{eventTotal.toLocaleString()}</p>
	</div>
	<div class="rounded-xl border border-border bg-surface p-5">
		<p class="mb-1 text-xs uppercase tracking-wider text-text-muted">Companies</p>
		<p class="font-mono text-3xl font-bold text-accent-glow">{companyTotal.toLocaleString()}</p>
	</div>
	<div class="rounded-xl border border-border bg-surface p-5">
		<p class="mb-1 text-xs uppercase tracking-wider text-text-muted">Users</p>
		<p class="font-mono text-3xl font-bold text-text">{userTotal.toLocaleString()}</p>
	</div>
	<div class="rounded-xl border border-border bg-surface p-5">
		<p class="mb-1 text-xs uppercase tracking-wider text-text-muted">Crawled Records</p>
		<p class="font-mono text-3xl font-bold text-text">{stats?.total?.toLocaleString() ?? '—'}</p>
	</div>
</div>

<!-- Crawl Activity -->
<div class="mb-6 grid grid-cols-3 gap-4">
	<div class="rounded-xl border border-border bg-surface p-5">
		<p class="mb-1 text-xs uppercase tracking-wider text-text-muted">Last 24h</p>
		<p class="font-mono text-2xl font-bold text-accent">{stats?.last_24h ?? '—'}</p>
	</div>
	<div class="rounded-xl border border-border bg-surface p-5">
		<p class="mb-1 text-xs uppercase tracking-wider text-text-muted">Last 7d</p>
		<p class="font-mono text-2xl font-bold text-text">{stats?.last_7d ?? '—'}</p>
	</div>
	<div class="rounded-xl border border-border bg-surface p-5">
		<p class="mb-1 text-xs uppercase tracking-wider text-text-muted">Last 30d</p>
		<p class="font-mono text-2xl font-bold text-text">{stats?.last_30d ?? '—'}</p>
	</div>
</div>

<div class="mb-6 grid gap-6 lg:grid-cols-3">
	<!-- Live Feed -->
	<div class="rounded-xl border border-border bg-surface p-5">
		<h2 class="mb-4 text-sm font-semibold uppercase tracking-wider text-text-muted">
			Live Feed
			<span class="ml-2 inline-block h-1.5 w-1.5 animate-pulse rounded-full bg-success"></span>
		</h2>
		<div class="max-h-72 space-y-2 overflow-y-auto">
			{#each liveEvents as event}
				<div class="rounded-lg bg-surface-hover px-3 py-2 font-mono text-xs">
					<span class="text-text-muted">{event.ts}</span>
					{#if event.type === 'crawl'}
						<span class="text-accent-glow"> ↓ CRAWL</span>
						<span class="text-text-muted">+{event.count} (total: {event.total})</span>
					{:else if event.type === 'review'}
						<span class="text-warning"> ★ REVIEW</span>
						<span class="text-text-muted">+{event.count} (total: {event.total})</span>
					{/if}
				</div>
			{/each}
			{#if liveEvents.length === 0}
				<p class="py-4 text-center text-xs text-text-muted">Waiting for events...</p>
			{/if}
		</div>
	</div>

	<!-- Trending -->
	<div class="rounded-xl border border-border bg-surface p-5">
		<h2 class="mb-4 text-sm font-semibold uppercase tracking-wider text-text-muted">Trending (30d)</h2>
		<div class="space-y-3">
			{#each trending as event, i}
				<div class="flex items-center gap-3">
					<span class="w-5 text-right font-mono text-xs text-text-muted">#{i + 1}</span>
					<div class="min-w-0 flex-1">
						<p class="truncate text-sm font-medium text-text">{event.name}</p>
						<p class="text-xs text-text-muted">
							{event.review_count} reviews · {event.avg_rating?.toFixed(1) ?? '—'}★
						</p>
					</div>
				</div>
			{/each}
			{#if trending.length === 0}
				<p class="py-4 text-center text-xs text-text-muted">No trending data</p>
			{/if}
		</div>
	</div>

	<!-- Rating Distribution -->
	<div class="rounded-xl border border-border bg-surface p-5">
		<h2 class="mb-4 text-sm font-semibold uppercase tracking-wider text-text-muted">Ratings</h2>
		<div class="space-y-2">
			{#each [5, 4, 3, 2, 1] as star}
				{@const bucket = ratings.find((r: any) => r.rating === star)}
				{@const count = bucket?.count ?? 0}
				{@const maxCount = Math.max(...ratings.map((r: any) => r.count ?? 0), 1)}
				<div class="flex items-center gap-3">
					<span class="w-6 text-right font-mono text-xs text-text-muted">{star}★</span>
					<div class="h-5 flex-1 overflow-hidden rounded-full bg-bg">
						<div
							class="h-full rounded-full transition-all duration-500"
							style="width: {(count / maxCount) * 100}%; background: var(--color-chart-{Math.min(star, 4)})"
						></div>
					</div>
					<span class="w-8 text-right font-mono text-xs text-text-muted">{count}</span>
				</div>
			{/each}
		</div>
	</div>
</div>

<!-- Recent Reviews -->
<div class="rounded-xl border border-border bg-surface p-5">
	<div class="mb-4 flex items-center justify-between">
		<h2 class="text-sm font-semibold uppercase tracking-wider text-text-muted">Recent Reviews</h2>
		<a href="/reviews" class="text-xs text-accent hover:text-accent-glow">View all →</a>
	</div>
	<div class="space-y-2">
		{#each reviews as review}
			<div class="flex items-center gap-4 rounded-lg px-3 py-2.5 transition-colors hover:bg-surface-hover">
				<span class="font-mono text-sm text-accent-glow">{'★'.repeat(review.rating)}{'☆'.repeat(5 - review.rating)}</span>
				<div class="min-w-0 flex-1">
					<span class="text-sm font-medium text-text">{review.title ?? 'Untitled'}</span>
					<span class="text-xs text-text-muted"> · {review.entity_name ?? 'Unknown'}</span>
				</div>
				<span class="text-xs text-text-muted">{review.username ?? 'anon'}</span>
				<span class="text-xs text-text-muted">{timeAgo(review.created_at)}</span>
			</div>
		{/each}
		{#if reviews.length === 0}
			<p class="py-4 text-center text-xs text-text-muted">No reviews yet</p>
		{/if}
	</div>
</div>
