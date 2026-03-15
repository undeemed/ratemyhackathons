<script lang="ts">
	import { onMount } from 'svelte';
	import { fetchCrawlSources, fetchCrawlStats, fetchCrawlHistory } from '$lib/api';

	let sources = $state<any[]>([]);
	let stats = $state<any>(null);
	let history = $state<any[]>([]);

	onMount(async () => {
		const results = await Promise.allSettled([
			fetchCrawlSources(),
			fetchCrawlStats(),
			fetchCrawlHistory(30),
		]);
		if (results[0].status === 'fulfilled') sources = results[0].value;
		if (results[1].status === 'fulfilled') stats = results[1].value;
		if (results[2].status === 'fulfilled') history = results[2].value;
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

	const maxHistoryCount = $derived(Math.max(...history.map((h: any) => h.count ?? 0), 1));
</script>

<h1 class="mb-6 text-xl font-semibold text-text">Crawl Sources</h1>

<!-- Stats -->
<div class="mb-6 grid grid-cols-2 gap-4 lg:grid-cols-4">
	<div class="rounded-xl border border-border bg-surface p-5">
		<p class="mb-1 text-xs uppercase tracking-wider text-text-muted">Total Crawled</p>
		<p class="font-mono text-3xl font-bold text-text">{stats?.total?.toLocaleString() ?? '—'}</p>
	</div>
	<div class="rounded-xl border border-border bg-surface p-5">
		<p class="mb-1 text-xs uppercase tracking-wider text-text-muted">Last 24h</p>
		<p class="font-mono text-3xl font-bold text-accent">{stats?.last_24h ?? '—'}</p>
	</div>
	<div class="rounded-xl border border-border bg-surface p-5">
		<p class="mb-1 text-xs uppercase tracking-wider text-text-muted">Last 7d</p>
		<p class="font-mono text-3xl font-bold text-text">{stats?.last_7d ?? '—'}</p>
	</div>
	<div class="rounded-xl border border-border bg-surface p-5">
		<p class="mb-1 text-xs uppercase tracking-wider text-text-muted">Last 30d</p>
		<p class="font-mono text-3xl font-bold text-text">{stats?.last_30d ?? '—'}</p>
	</div>
</div>

<!-- By Source Type -->
{#if stats?.by_source?.length}
	<div class="mb-6 rounded-xl border border-border bg-surface p-5">
		<h2 class="mb-4 text-sm font-semibold uppercase tracking-wider text-text-muted">By Source Type</h2>
		<div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-4">
			{#each stats.by_source as src}
				<div class="flex items-center gap-3 rounded-lg bg-surface-hover px-4 py-3">
					<span class="rounded bg-accent/15 px-2 py-0.5 font-mono text-xs text-accent-glow">{src.source_type}</span>
					<span class="font-mono text-lg font-bold text-text">{src.count.toLocaleString()}</span>
				</div>
			{/each}
		</div>
	</div>
{/if}

<!-- Source Table -->
<div class="mb-6 overflow-hidden rounded-xl border border-border bg-surface">
	<table class="w-full text-sm">
		<thead>
			<tr class="border-b border-border text-left text-text-muted">
				<th class="px-4 py-3 font-medium">Source</th>
				<th class="px-4 py-3 font-medium">Type</th>
				<th class="px-4 py-3 font-medium">Base URL</th>
				<th class="px-4 py-3 font-medium text-right">Events</th>
				<th class="px-4 py-3 font-medium">Poll Interval</th>
				<th class="px-4 py-3 font-medium">Last Polled</th>
				<th class="px-4 py-3 font-medium text-center">Status</th>
			</tr>
		</thead>
		<tbody>
			{#each sources as source}
				<tr class="border-b border-border/50 transition-colors hover:bg-surface-hover">
					<td class="px-4 py-3 font-medium text-text">{source.name}</td>
					<td class="px-4 py-3">
						<span class="rounded bg-accent/15 px-2 py-0.5 font-mono text-xs text-accent-glow">{source.source_type}</span>
					</td>
					<td class="px-4 py-3 font-mono text-xs text-text-muted">{source.base_url}</td>
					<td class="px-4 py-3 text-right font-mono text-text-muted">{source.event_count ?? 0}</td>
					<td class="px-4 py-3 font-mono text-xs text-text-muted">{source.poll_interval_hours}h</td>
					<td class="px-4 py-3 text-xs text-text-muted">
						{source.last_polled_at ? timeAgo(source.last_polled_at) : 'Never'}
					</td>
					<td class="px-4 py-3 text-center">
						<span class="inline-block h-2.5 w-2.5 rounded-full {source.enabled ? 'bg-success' : 'bg-danger'}"></span>
					</td>
				</tr>
			{/each}
			{#if sources.length === 0}
				<tr><td colspan="7" class="py-8 text-center text-text-muted">No sources configured</td></tr>
			{/if}
		</tbody>
	</table>
</div>

<!-- Crawl History (30d sparkline) -->
{#if history.length > 0}
	<div class="rounded-xl border border-border bg-surface p-5">
		<h2 class="mb-4 text-sm font-semibold uppercase tracking-wider text-text-muted">Crawl History (30d)</h2>
		<div class="flex h-32 items-end gap-1">
			{#each history as day}
				{@const pct = ((day.count ?? 0) / maxHistoryCount) * 100}
				<div
					class="flex-1 rounded-t bg-accent transition-all hover:bg-accent-glow"
					style="height: {Math.max(pct, 2)}%"
					title="{day.day}: {day.count} records"
				></div>
			{/each}
		</div>
		<div class="mt-2 flex justify-between text-[10px] text-text-muted">
			<span>{history[0]?.day ?? ''}</span>
			<span>{history[history.length - 1]?.day ?? ''}</span>
		</div>
	</div>
{/if}
