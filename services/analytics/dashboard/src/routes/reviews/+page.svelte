<script lang="ts">
	import { onMount } from 'svelte';
	import { fetchRecentReviews, fetchRatingDistribution, fetchTrending } from '$lib/api';

	let reviews = $state<any[]>([]);
	let ratings = $state<any[]>([]);
	let trending = $state<any[]>([]);

	onMount(async () => {
		const results = await Promise.allSettled([
			fetchRecentReviews(30),
			fetchRatingDistribution(),
			fetchTrending(30, 10),
		]);
		if (results[0].status === 'fulfilled') reviews = results[0].value;
		if (results[1].status === 'fulfilled') ratings = results[1].value;
		if (results[2].status === 'fulfilled') trending = results[2].value;
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

	const totalReviews = $derived(ratings.reduce((sum: number, r: any) => sum + (r.count ?? 0), 0));
	const avgRating = $derived(
		totalReviews > 0
			? (ratings.reduce((sum: number, r: any) => sum + (r.rating ?? 0) * (r.count ?? 0), 0) / totalReviews).toFixed(2)
			: '—'
	);
</script>

<h1 class="mb-6 text-xl font-semibold text-text">Reviews</h1>

<!-- Quick Stats -->
<div class="mb-6 grid grid-cols-2 gap-4 lg:grid-cols-3">
	<div class="rounded-xl border border-border bg-surface p-5">
		<p class="mb-1 text-xs uppercase tracking-wider text-text-muted">Total Reviews</p>
		<p class="font-mono text-3xl font-bold text-text">{totalReviews}</p>
	</div>
	<div class="rounded-xl border border-border bg-surface p-5">
		<p class="mb-1 text-xs uppercase tracking-wider text-text-muted">Average Rating</p>
		<p class="font-mono text-3xl font-bold text-accent-glow">{avgRating}★</p>
	</div>
	<div class="col-span-2 rounded-xl border border-border bg-surface p-5 lg:col-span-1">
		<p class="mb-1 text-xs uppercase tracking-wider text-text-muted">Most Reviewed</p>
		<p class="truncate text-lg font-bold text-text">{trending[0]?.name ?? '—'}</p>
		<p class="text-xs text-text-muted">{trending[0]?.review_count ?? 0} reviews</p>
	</div>
</div>

<div class="mb-6 grid gap-6 lg:grid-cols-2">
	<!-- Rating Distribution -->
	<div class="rounded-xl border border-border bg-surface p-5">
		<h2 class="mb-4 text-sm font-semibold uppercase tracking-wider text-text-muted">Rating Distribution</h2>
		<div class="space-y-3">
			{#each [5, 4, 3, 2, 1] as star}
				{@const bucket = ratings.find((r: any) => r.rating === star)}
				{@const count = bucket?.count ?? 0}
				{@const pct = totalReviews > 0 ? ((count / totalReviews) * 100).toFixed(1) : '0'}
				<div class="flex items-center gap-3">
					<span class="w-6 text-right font-mono text-sm text-text-muted">{star}★</span>
					<div class="h-6 flex-1 overflow-hidden rounded-full bg-bg">
						<div
							class="flex h-full items-center rounded-full px-2 transition-all duration-500"
							style="width: {Math.max(Number(pct), 2)}%; background: var(--color-chart-{Math.min(star, 4)})"
						>
							{#if Number(pct) > 10}
								<span class="font-mono text-[10px] text-white">{pct}%</span>
							{/if}
						</div>
					</div>
					<span class="w-10 text-right font-mono text-xs text-text-muted">{count}</span>
				</div>
			{/each}
		</div>
	</div>

	<!-- Trending Events -->
	<div class="rounded-xl border border-border bg-surface p-5">
		<h2 class="mb-4 text-sm font-semibold uppercase tracking-wider text-text-muted">Most Reviewed (30d)</h2>
		<div class="space-y-2">
			{#each trending as event, i}
				<div class="flex items-center gap-3 rounded-lg px-3 py-2 transition-colors hover:bg-surface-hover">
					<span class="w-5 text-right font-mono text-xs text-text-muted">#{i + 1}</span>
					<div class="min-w-0 flex-1">
						<p class="truncate text-sm font-medium text-text">{event.name}</p>
					</div>
					<span class="font-mono text-xs text-text-muted">{event.review_count} rev</span>
					{#if event.avg_rating}
						<span class="font-mono text-sm font-bold {event.avg_rating >= 4 ? 'text-success' : event.avg_rating >= 3 ? 'text-warning' : 'text-danger'}">
							{event.avg_rating.toFixed(1)}
						</span>
					{/if}
				</div>
			{/each}
			{#if trending.length === 0}
				<p class="py-4 text-center text-xs text-text-muted">No data</p>
			{/if}
		</div>
	</div>
</div>

<!-- All Reviews -->
<div class="overflow-hidden rounded-xl border border-border bg-surface">
	<div class="border-b border-border px-5 py-3">
		<h2 class="text-sm font-semibold uppercase tracking-wider text-text-muted">Recent Reviews</h2>
	</div>
	<table class="w-full text-sm">
		<thead>
			<tr class="border-b border-border text-left text-text-muted">
				<th class="px-4 py-3 font-medium">Rating</th>
				<th class="px-4 py-3 font-medium">Title</th>
				<th class="px-4 py-3 font-medium">Event / Company</th>
				<th class="px-4 py-3 font-medium">User</th>
				<th class="px-4 py-3 font-medium">When</th>
			</tr>
		</thead>
		<tbody>
			{#each reviews as review}
				<tr class="border-b border-border/50 transition-colors hover:bg-surface-hover">
					<td class="px-4 py-3">
						<span class="font-mono text-sm {review.rating >= 4 ? 'text-success' : review.rating >= 3 ? 'text-warning' : 'text-danger'}">
							{'★'.repeat(review.rating)}{'☆'.repeat(5 - review.rating)}
						</span>
					</td>
					<td class="max-w-[200px] truncate px-4 py-3 font-medium text-text">{review.title ?? 'Untitled'}</td>
					<td class="px-4 py-3 text-text-muted">{review.entity_name ?? 'Unknown'}</td>
					<td class="px-4 py-3 font-mono text-xs text-text-muted">{review.username ?? 'anon'}</td>
					<td class="px-4 py-3 text-xs text-text-muted">{timeAgo(review.created_at)}</td>
				</tr>
			{/each}
			{#if reviews.length === 0}
				<tr><td colspan="5" class="py-8 text-center text-text-muted">No reviews yet</td></tr>
			{/if}
		</tbody>
	</table>
</div>
