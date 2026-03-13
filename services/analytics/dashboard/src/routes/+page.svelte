<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import {
		fetchCrawlStats,
		fetchCrawlSources,
		fetchCrawlHistory,
		fetchRatingDistribution,
		fetchRecentReviews,
		fetchTrending,
		connectLiveFeed
	} from '$lib/api';

	let stats = $state<any>(null);
	let sources = $state<any[]>([]);
	let history = $state<any[]>([]);
	let ratings = $state<any[]>([]);
	let reviews = $state<any[]>([]);
	let trending = $state<any[]>([]);
	let liveEvents = $state<any[]>([]);
	let liveFeed: EventSource | null = $state(null);

	onMount(async () => {
		const results = await Promise.allSettled([
			fetchCrawlStats(),
			fetchCrawlSources(),
			fetchCrawlHistory(30),
			fetchRatingDistribution(),
			fetchRecentReviews(10),
			fetchTrending(30, 10),
		]);

		if (results[0].status === 'fulfilled') stats = results[0].value;
		if (results[1].status === 'fulfilled') sources = results[1].value;
		if (results[2].status === 'fulfilled') history = results[2].value;
		if (results[3].status === 'fulfilled') ratings = results[3].value;
		if (results[4].status === 'fulfilled') reviews = results[4].value;
		if (results[5].status === 'fulfilled') trending = results[5].value;

		liveFeed = connectLiveFeed((event) => {
			liveEvents = [event, ...liveEvents.slice(0, 49)];
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

	function ratingStars(n: number): string {
		return '★'.repeat(n) + '☆'.repeat(5 - n);
	}
</script>

<!-- Stat Cards -->
<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
	<div class="bg-surface rounded-xl border border-border p-5">
		<p class="text-xs text-text-muted uppercase tracking-wider mb-1">Total Scraped</p>
		<p class="text-3xl font-bold text-text font-mono">{stats?.total ?? '—'}</p>
	</div>
	<div class="bg-surface rounded-xl border border-border p-5">
		<p class="text-xs text-text-muted uppercase tracking-wider mb-1">Last 24h</p>
		<p class="text-3xl font-bold text-accent font-mono">{stats?.last_24h ?? '—'}</p>
	</div>
	<div class="bg-surface rounded-xl border border-border p-5">
		<p class="text-xs text-text-muted uppercase tracking-wider mb-1">Last 7d</p>
		<p class="text-3xl font-bold text-text font-mono">{stats?.last_7d ?? '—'}</p>
	</div>
	<div class="bg-surface rounded-xl border border-border p-5">
		<p class="text-xs text-text-muted uppercase tracking-wider mb-1">Last 30d</p>
		<p class="text-3xl font-bold text-text font-mono">{stats?.last_30d ?? '—'}</p>
	</div>
</div>

<div class="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
	<!-- Source Health -->
	<div class="lg:col-span-2 bg-surface rounded-xl border border-border p-5">
		<h2 class="text-sm font-semibold text-text-muted uppercase tracking-wider mb-4">Source Health</h2>
		<div class="overflow-x-auto">
			<table class="w-full text-sm">
				<thead>
					<tr class="text-left text-text-muted border-b border-border">
						<th class="pb-2 font-medium">Source</th>
						<th class="pb-2 font-medium">Type</th>
						<th class="pb-2 font-medium">Events</th>
						<th class="pb-2 font-medium">Last Poll</th>
						<th class="pb-2 font-medium">Status</th>
					</tr>
				</thead>
				<tbody>
					{#each sources as source}
						<tr class="border-b border-border/50 hover:bg-surface-hover transition-colors">
							<td class="py-3 text-text font-medium">{source.name}</td>
							<td class="py-3">
								<span class="px-2 py-0.5 rounded-full text-xs bg-accent/15 text-accent-glow font-mono">
									{source.source_type}
								</span>
							</td>
							<td class="py-3 font-mono text-text-muted">{source.event_count ?? 0}</td>
							<td class="py-3 text-text-muted text-xs">
								{source.last_polled_at ? timeAgo(source.last_polled_at) : 'Never'}
							</td>
							<td class="py-3">
								<span class="w-2 h-2 rounded-full inline-block {source.enabled ? 'bg-success' : 'bg-danger'}"></span>
							</td>
						</tr>
					{/each}
					{#if sources.length === 0}
						<tr><td colspan="5" class="py-8 text-center text-text-muted">No sources configured</td></tr>
					{/if}
				</tbody>
			</table>
		</div>
	</div>

	<!-- Live Feed -->
	<div class="bg-surface rounded-xl border border-border p-5">
		<h2 class="text-sm font-semibold text-text-muted uppercase tracking-wider mb-4">
			Live Feed
			<span class="ml-2 w-1.5 h-1.5 rounded-full bg-success inline-block animate-pulse"></span>
		</h2>
		<div class="space-y-2 max-h-80 overflow-y-auto">
			{#each liveEvents as event}
				<div class="px-3 py-2 rounded-lg bg-surface-hover text-xs font-mono">
					{#if event.type === 'crawl'}
						<span class="text-accent-glow">↓ CRAWL</span>
						<span class="text-text-muted">+{event.count} events (total: {event.total})</span>
					{:else if event.type === 'review'}
						<span class="text-warning">★ REVIEW</span>
						<span class="text-text-muted">+{event.count} reviews (total: {event.total})</span>
					{/if}
				</div>
			{/each}
			{#if liveEvents.length === 0}
				<p class="text-text-muted text-xs text-center py-4">Waiting for events...</p>
			{/if}
		</div>
	</div>
</div>

<div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
	<!-- Rating Distribution -->
	<div class="bg-surface rounded-xl border border-border p-5">
		<h2 class="text-sm font-semibold text-text-muted uppercase tracking-wider mb-4">Rating Distribution</h2>
		<div class="space-y-2">
			{#each [5, 4, 3, 2, 1] as star}
				{@const bucket = ratings.find((r: any) => r.rating === star)}
				{@const count = bucket?.count ?? 0}
				{@const maxCount = Math.max(...ratings.map((r: any) => r.count ?? 0), 1)}
				<div class="flex items-center gap-3">
					<span class="text-xs font-mono w-6 text-text-muted text-right">{star}★</span>
					<div class="flex-1 h-5 bg-bg rounded-full overflow-hidden">
						<div
							class="h-full rounded-full transition-all duration-500"
							style="width: {(count / maxCount) * 100}%; background: var(--color-chart-{Math.min(star, 4)})"
						></div>
					</div>
					<span class="text-xs font-mono w-8 text-text-muted text-right">{count}</span>
				</div>
			{/each}
		</div>
	</div>

	<!-- Trending Events -->
	<div class="bg-surface rounded-xl border border-border p-5">
		<h2 class="text-sm font-semibold text-text-muted uppercase tracking-wider mb-4">Trending Events (30d)</h2>
		<div class="space-y-3">
			{#each trending as event, i}
				<div class="flex items-center gap-3 group">
					<span class="text-xs font-mono text-text-muted w-5 text-right">#{i + 1}</span>
					<div class="flex-1 min-w-0">
						<p class="text-sm text-text font-medium truncate group-hover:text-accent-glow transition-colors">
							{event.name}
						</p>
						<p class="text-xs text-text-muted">
							{event.review_count} reviews · {event.avg_rating?.toFixed(1) ?? '—'}★
						</p>
					</div>
				</div>
			{/each}
			{#if trending.length === 0}
				<p class="text-text-muted text-xs text-center py-4">No trending data yet</p>
			{/if}
		</div>
	</div>
</div>

<!-- Recent Reviews -->
<div class="bg-surface rounded-xl border border-border p-5">
	<h2 class="text-sm font-semibold text-text-muted uppercase tracking-wider mb-4">Recent Reviews</h2>
	<div class="space-y-3">
		{#each reviews as review}
			<div class="flex items-start gap-4 px-3 py-3 rounded-lg hover:bg-surface-hover transition-colors">
				<div class="flex-1 min-w-0">
					<div class="flex items-center gap-2 mb-1">
						<span class="text-accent-glow text-sm tracking-wider">{ratingStars(review.rating)}</span>
						<span class="text-xs text-text-muted">by {review.username ?? 'anon'}</span>
					</div>
					<p class="text-sm text-text font-medium">{review.title}</p>
					<p class="text-xs text-text-muted mt-0.5">{review.event_name} · {timeAgo(review.created_at)}</p>
				</div>
			</div>
		{/each}
		{#if reviews.length === 0}
			<p class="text-text-muted text-xs text-center py-4">No reviews yet</p>
		{/if}
	</div>
</div>
