<script lang="ts">
	import { onMount } from 'svelte';
	import { fetchEvents } from '$lib/api';

	let events = $state<any[]>([]);
	let total = $state(0);
	let page = $state(1);
	let perPage = 50;
	let loading = $state(true);
	let search = $state('');

	async function load() {
		loading = true;
		try {
			const res = await fetchEvents(page, perPage);
			events = res.data ?? [];
			total = res.total ?? 0;
		} catch {
			events = [];
		}
		loading = false;
	}

	onMount(load);

	function nextPage() {
		if (page * perPage < total) { page++; load(); }
	}
	function prevPage() {
		if (page > 1) { page--; load(); }
	}

	const filtered = $derived(
		search
			? events.filter(e =>
				e.name?.toLowerCase().includes(search.toLowerCase()) ||
				e.location?.toLowerCase().includes(search.toLowerCase())
			)
			: events
	);

	function fmtDate(d: string | null) {
		if (!d) return '—';
		return new Date(d).toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
	}
</script>

<div class="mb-6 flex items-center justify-between">
	<h1 class="text-xl font-semibold text-text">Events <span class="font-mono text-sm text-text-muted">({total.toLocaleString()})</span></h1>
	<input
		bind:value={search}
		type="text"
		placeholder="Filter events..."
		class="rounded-lg border border-border bg-surface px-4 py-2 text-sm text-text placeholder:text-text-muted focus:border-accent focus:outline-none"
	/>
</div>

{#if loading}
	<p class="py-12 text-center text-sm text-text-muted">Loading...</p>
{:else}
	<div class="overflow-hidden rounded-xl border border-border bg-surface">
		<table class="w-full text-sm">
			<thead>
				<tr class="border-b border-border text-left text-text-muted">
					<th class="px-4 py-3 font-medium">Name</th>
					<th class="px-4 py-3 font-medium">Location</th>
					<th class="px-4 py-3 font-medium">Date</th>
					<th class="px-4 py-3 font-medium text-right">Rating</th>
					<th class="px-4 py-3 font-medium text-right">Reviews</th>
					<th class="px-4 py-3 font-medium">Companies</th>
				</tr>
			</thead>
			<tbody>
				{#each filtered as event}
					<tr class="border-b border-border/50 transition-colors hover:bg-surface-hover">
						<td class="px-4 py-3">
							<a href="https://ratemyhackathons.com/events/{event.id}" target="_blank"
								class="font-medium text-text hover:text-accent-glow">
								{event.name}
							</a>
						</td>
						<td class="px-4 py-3 text-text-muted">{event.location ?? '—'}</td>
						<td class="px-4 py-3 font-mono text-xs text-text-muted">{fmtDate(event.start_date)}</td>
						<td class="px-4 py-3 text-right">
							{#if event.avg_rating}
								<span class="font-mono font-bold {event.avg_rating >= 4 ? 'text-success' : event.avg_rating >= 3 ? 'text-warning' : 'text-danger'}">
									{event.avg_rating.toFixed(1)}
								</span>
							{:else}
								<span class="text-text-muted">—</span>
							{/if}
						</td>
						<td class="px-4 py-3 text-right font-mono text-text-muted">{event.review_count ?? 0}</td>
						<td class="px-4 py-3">
							{#if event.companies?.length}
								<div class="flex flex-wrap gap-1">
									{#each event.companies.slice(0, 3) as c}
										<span class="rounded bg-accent/15 px-1.5 py-0.5 font-mono text-[10px] text-accent-glow">{c.name}</span>
									{/each}
									{#if event.companies.length > 3}
										<span class="text-[10px] text-text-muted">+{event.companies.length - 3}</span>
									{/if}
								</div>
							{:else}
								<span class="text-text-muted">—</span>
							{/if}
						</td>
					</tr>
				{/each}
				{#if filtered.length === 0}
					<tr><td colspan="6" class="py-8 text-center text-text-muted">No events found</td></tr>
				{/if}
			</tbody>
		</table>
	</div>

	<!-- Pagination -->
	<div class="mt-4 flex items-center justify-between">
		<p class="text-xs text-text-muted">
			Showing {(page - 1) * perPage + 1}–{Math.min(page * perPage, total)} of {total.toLocaleString()}
		</p>
		<div class="flex gap-2">
			<button onclick={prevPage} disabled={page <= 1}
				class="rounded-lg border border-border px-3 py-1.5 text-xs text-text-muted transition-colors hover:bg-surface-hover disabled:opacity-30">
				← Prev
			</button>
			<span class="rounded-lg border border-border bg-surface-hover px-3 py-1.5 font-mono text-xs text-text">{page}</span>
			<button onclick={nextPage} disabled={page * perPage >= total}
				class="rounded-lg border border-border px-3 py-1.5 text-xs text-text-muted transition-colors hover:bg-surface-hover disabled:opacity-30">
				Next →
			</button>
		</div>
	</div>
{/if}
