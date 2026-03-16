<script lang="ts">
	import { search } from '$lib/api';
	import { goto } from '$app/navigation';
	import { scoreColor } from '$lib/types';

	let { mode = 'events', placeholder = 'Search hackathons...' }: {
		mode?: 'events' | 'companies';
		placeholder?: string;
	} = $props();

	let query = $state('');
	let results = $state<{ id: string; name: string; avg_rating: number | null; review_count: number; type: 'event' | 'company' }[]>([]);
	let open = $state(false);
	let loading = $state(false);
	let selectedIndex = $state(-1);
	let debounceTimer: ReturnType<typeof setTimeout> | undefined;
	let inputEl: HTMLInputElement;

	function debounceSearch() {
		clearTimeout(debounceTimer);
		selectedIndex = -1;

		if (!query.trim()) {
			results = [];
			open = false;
			return;
		}

		loading = true;
		debounceTimer = setTimeout(async () => {
			try {
				const searchType = mode === 'companies' ? 'company' : 'event';
				const res = await search(query.trim(), searchType);
				const items: typeof results = [];

				if (mode === 'events') {
					for (const e of res.events.slice(0, 6)) {
						items.push({ id: e.id, name: e.name, avg_rating: e.avg_rating, review_count: e.review_count, type: 'event' });
					}
				} else {
					for (const c of res.companies.slice(0, 6)) {
						items.push({ id: c.id, name: c.name, avg_rating: c.avg_rating, review_count: c.review_count, type: 'company' });
					}
				}

				results = items;
				open = items.length > 0;
			} catch {
				results = [];
				open = false;
			} finally {
				loading = false;
			}
		}, 300);
	}

	function navigateToResult(item: typeof results[0]) {
		const path = item.type === 'event' ? `/events/${item.id}` : `/companies/${item.id}`;
		open = false;
		query = '';
		goto(path);
	}

	function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		if (selectedIndex >= 0 && results[selectedIndex]) {
			navigateToResult(results[selectedIndex]);
			return;
		}
		if (query.trim()) {
			const type = mode === 'companies' ? '&type=company' : '';
			open = false;
			goto(`/search?q=${encodeURIComponent(query.trim())}${type}`);
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (!open || results.length === 0) return;

		if (e.key === 'ArrowDown') {
			e.preventDefault();
			selectedIndex = Math.min(selectedIndex + 1, results.length - 1);
		} else if (e.key === 'ArrowUp') {
			e.preventDefault();
			selectedIndex = Math.max(selectedIndex - 1, -1);
		} else if (e.key === 'Escape') {
			open = false;
			selectedIndex = -1;
		}
	}

	function handleBlur() {
		// Delay so click on dropdown item registers before closing
		setTimeout(() => { open = false; }, 200);
	}

	function ratingColor(avg: number | null): string {
		if (avg === null) return '#555';
		return scoreColor(avg);
	}
</script>

<form onsubmit={handleSubmit} class="relative">
	<div class="group flex items-center border-b-2 border-dim transition-colors focus-within:border-text">
		<input
			bind:this={inputEl}
			bind:value={query}
			oninput={debounceSearch}
			onkeydown={handleKeydown}
			onfocus={() => { if (results.length > 0) open = true; }}
			onblur={handleBlur}
			type="text"
			{placeholder}
			class="w-full bg-transparent py-4 text-lg text-text placeholder:text-dim focus:outline-none"
		/>
		{#if loading}
			<span class="mr-2 text-xs text-dim">...</span>
		{/if}
		<button type="submit" class="text-dim transition-colors group-focus-within:text-text" aria-label="Search">
			<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="5" y1="12" x2="19" y2="12"></line><polyline points="12 5 19 12 12 19"></polyline></svg>
		</button>
	</div>

	{#if open && results.length > 0}
		<div class="absolute left-0 right-0 top-full z-50 border border-border bg-bg shadow-lg">
			{#each results as item, i}
				<button
					type="button"
					onmousedown={() => navigateToResult(item)}
					class="flex w-full items-center gap-3 px-4 py-3 text-left transition-colors hover:bg-elevated {i === selectedIndex ? 'bg-elevated' : ''}"
				>
					<!-- Score badge -->
					<span
						class="flex h-8 w-8 shrink-0 items-center justify-center text-xs font-bold"
						style="background-color: {ratingColor(item.avg_rating)}20; color: {ratingColor(item.avg_rating)}"
					>
						{item.avg_rating !== null ? item.avg_rating.toFixed(1) : '—'}
					</span>

					<div class="min-w-0 flex-1">
						<p class="truncate font-display text-sm italic text-text">{item.name}</p>
						<p class="text-[10px] tracking-[0.2em] text-dim">
							{item.review_count} review{item.review_count !== 1 ? 's' : ''}
						</p>
					</div>

					<span class="text-[9px] uppercase tracking-[0.2em] text-dim">
						{item.type === 'event' ? 'Event' : 'Company'}
					</span>
				</button>
			{/each}

			<a
				href="/search?q={encodeURIComponent(query.trim())}"
				class="block border-t border-border px-4 py-2 text-center text-[10px] uppercase tracking-[0.2em] text-dim transition-colors hover:text-text"
			>
				View all results &rarr;
			</a>
		</div>
	{/if}
</form>
