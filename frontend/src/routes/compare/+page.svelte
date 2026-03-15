<script lang="ts">
	import { goto } from '$app/navigation';
	import { onDestroy } from 'svelte';
	import ScoreBadge from '$lib/components/ScoreBadge.svelte';
	import { fadeIn } from '$lib/animations/gsap';
	import { scoreColor, CATEGORY_LABELS, type RatingCategory } from '$lib/types';
	import type { CompareEntity, SearchResult } from '$lib/types';
	import type { PageData } from './$types';
	import { search as searchApi } from '$lib/api';
	import { DEV } from 'esm-env';

	let { data }: { data: PageData } = $props();

	// ── Search + selection state ──
	let searchQuery = $state('');
	let searchType = $state<'event' | 'company'>('event');
	$effect(() => { searchType = data.type ?? 'event'; });
	let searchResults = $state<SearchResult[]>([]);
	let searching = $state(false);
	let showDropdown = $state(false);
	let debounceTimer: ReturnType<typeof setTimeout>;
	let wrapperEl = $state<HTMLElement>();

	// Name cache — remembers names from search selections and loaded entities
	const nameCache = new Map<string, string>();

	$effect(() => {
		for (const e of data.entities) {
			nameCache.set(e.id, e.name);
		}
	});

	let chips = $derived(
		data.ids.map((id) => ({
			id,
			name: data.entities.find((e) => e.id === id)?.name ?? nameCache.get(id) ?? id.slice(0, 8) + '\u2026',
		}))
	);

	onDestroy(() => clearTimeout(debounceTimer));

	function handleInput() {
		clearTimeout(debounceTimer);
		const q = searchQuery.trim();
		if (!q) {
			searchResults = [];
			showDropdown = false;
			return;
		}
		debounceTimer = setTimeout(async () => {
			searching = true;
			try {
				const results = await searchApi(q, searchType);
				const pool = searchType === 'event' ? results.events : results.companies;
				searchResults = pool.filter((r) => !data.ids.includes(r.id));
				showDropdown = true;
			} catch {
				searchResults = [];
				showDropdown = false;
			}
			searching = false;
		}, 300);
	}

	function addEntity(id: string, name: string) {
		nameCache.set(id, name);
		const newIds = [...data.ids, id];
		showDropdown = false;
		searchQuery = '';
		searchResults = [];
		goto(`/compare?type=${searchType}&ids=${newIds.join(',')}`, { replaceState: true, noScroll: true });
	}

	function removeEntity(id: string) {
		nameCache.delete(id);
		const newIds = data.ids.filter((i) => i !== id);
		if (newIds.length === 0) {
			goto(`/compare?type=${searchType}`, { replaceState: true, noScroll: true });
		} else {
			goto(`/compare?type=${searchType}&ids=${newIds.join(',')}`, { replaceState: true, noScroll: true });
		}
	}

	function switchType(type: 'event' | 'company') {
		if (type === searchType) return;
		searchType = type;
		searchQuery = '';
		searchResults = [];
		showDropdown = false;
		nameCache.clear();
		goto(`/compare?type=${type}`, { replaceState: true, noScroll: true });
	}

	function handleWindowClick(e: MouseEvent) {
		if (wrapperEl && !wrapperEl.contains(e.target as Node)) {
			showDropdown = false;
		}
	}

	const allCategories = $derived(() => {
		const cats = new Set<string>();
		for (const entity of data.entities) {
			for (const cr of entity.category_ratings) {
				cats.add(cr.category);
			}
		}
		const order = Object.keys(CATEGORY_LABELS);
		return [...cats].sort((a, b) => order.indexOf(a) - order.indexOf(b));
	});

	// ── Dev-only mock data ──
	const MOCK_EVENTS: CompareEntity[] = DEV
		? [
				{
					id: 'demo-ev-1',
					name: 'HackMIT 2025',
					avg_rating: 4.3,
					review_count: 87,
					would_return_pct: 91,
					category_ratings: [
						{ category: 'organization', avg: 4.5 },
						{ category: 'prizes', avg: 4.7 },
						{ category: 'mentorship', avg: 3.9 },
						{ category: 'judging', avg: 3.6 },
						{ category: 'venue', avg: 4.8 },
						{ category: 'food', avg: 4.2 },
						{ category: 'swag', avg: 3.8 },
						{ category: 'networking', avg: 4.4 },
						{ category: 'communication', avg: 4.1 },
						{ category: 'vibes', avg: 4.6 },
					],
				},
				{
					id: 'demo-ev-2',
					name: 'TreeHacks 2025',
					avg_rating: 4.1,
					review_count: 64,
					would_return_pct: 85,
					category_ratings: [
						{ category: 'organization', avg: 4.2 },
						{ category: 'prizes', avg: 3.9 },
						{ category: 'mentorship', avg: 4.3 },
						{ category: 'judging', avg: 4.0 },
						{ category: 'venue', avg: 4.6 },
						{ category: 'food', avg: 4.5 },
						{ category: 'swag', avg: 4.1 },
						{ category: 'networking', avg: 3.7 },
						{ category: 'communication', avg: 3.8 },
						{ category: 'vibes', avg: 4.3 },
					],
				},
				{
					id: 'demo-ev-3',
					name: 'CalHacks 2025',
					avg_rating: 3.8,
					review_count: 52,
					would_return_pct: 72,
					category_ratings: [
						{ category: 'organization', avg: 3.5 },
						{ category: 'prizes', avg: 4.2 },
						{ category: 'mentorship', avg: 3.4 },
						{ category: 'judging', avg: 3.1 },
						{ category: 'venue', avg: 4.0 },
						{ category: 'food', avg: 3.6 },
						{ category: 'swag', avg: 3.9 },
						{ category: 'networking', avg: 4.1 },
						{ category: 'communication', avg: 3.3 },
						{ category: 'vibes', avg: 4.0 },
					],
				},
				{
					id: 'demo-ev-4',
					name: 'Hack the North 2025',
					avg_rating: 4.6,
					review_count: 143,
					would_return_pct: 94,
					category_ratings: [
						{ category: 'organization', avg: 4.8 },
						{ category: 'prizes', avg: 4.4 },
						{ category: 'mentorship', avg: 4.5 },
						{ category: 'judging', avg: 4.2 },
						{ category: 'venue', avg: 4.9 },
						{ category: 'food', avg: 4.7 },
						{ category: 'swag', avg: 4.3 },
						{ category: 'networking', avg: 4.6 },
						{ category: 'communication', avg: 4.4 },
						{ category: 'vibes', avg: 4.8 },
					],
				},
			]
		: [];

	const MOCK_COMPANIES: CompareEntity[] = DEV
		? [
				{
					id: 'demo-co-1',
					name: 'Major League Hacking',
					avg_rating: 4.4,
					review_count: 210,
					would_return_pct: 88,
					category_ratings: [
						{ category: 'organization', avg: 4.6 },
						{ category: 'prizes', avg: 4.0 },
						{ category: 'mentorship', avg: 4.5 },
						{ category: 'judging', avg: 4.1 },
						{ category: 'venue', avg: 4.3 },
						{ category: 'food', avg: 3.9 },
						{ category: 'swag', avg: 4.7 },
						{ category: 'networking', avg: 4.2 },
						{ category: 'communication', avg: 4.5 },
						{ category: 'vibes', avg: 4.4 },
					],
				},
				{
					id: 'demo-co-2',
					name: 'ETHGlobal',
					avg_rating: 4.2,
					review_count: 156,
					would_return_pct: 82,
					category_ratings: [
						{ category: 'organization', avg: 4.3 },
						{ category: 'prizes', avg: 4.8 },
						{ category: 'mentorship', avg: 3.7 },
						{ category: 'judging', avg: 3.9 },
						{ category: 'venue', avg: 4.5 },
						{ category: 'food', avg: 4.1 },
						{ category: 'swag', avg: 4.4 },
						{ category: 'networking', avg: 4.6 },
						{ category: 'communication', avg: 3.8 },
						{ category: 'vibes', avg: 4.3 },
					],
				},
				{
					id: 'demo-co-3',
					name: 'Devpost',
					avg_rating: 3.6,
					review_count: 98,
					would_return_pct: 68,
					category_ratings: [
						{ category: 'organization', avg: 3.4 },
						{ category: 'prizes', avg: 3.8 },
						{ category: 'mentorship', avg: 3.2 },
						{ category: 'judging', avg: 3.5 },
						{ category: 'venue', avg: 3.7 },
						{ category: 'food', avg: 3.3 },
						{ category: 'swag', avg: 3.6 },
						{ category: 'networking', avg: 3.9 },
						{ category: 'communication', avg: 3.4 },
						{ category: 'vibes', avg: 3.7 },
					],
				},
				{
					id: 'demo-co-4',
					name: 'Cerebral Valley',
					avg_rating: 4.5,
					review_count: 73,
					would_return_pct: 93,
					category_ratings: [
						{ category: 'organization', avg: 4.4 },
						{ category: 'prizes', avg: 4.2 },
						{ category: 'mentorship', avg: 4.7 },
						{ category: 'judging', avg: 4.3 },
						{ category: 'venue', avg: 4.6 },
						{ category: 'food', avg: 4.8 },
						{ category: 'swag', avg: 4.0 },
						{ category: 'networking', avg: 4.9 },
						{ category: 'communication', avg: 4.5 },
						{ category: 'vibes', avg: 4.7 },
					],
				},
			]
		: [];

	let mockType = $state<'event' | 'company'>('event');
	$effect(() => { mockType = data.type ?? 'event'; });
	let mockPool = $derived(mockType === 'event' ? MOCK_EVENTS : MOCK_COMPANIES);
	let mockSelected = $state<Set<string>>(new Set());
	let mockEntities = $derived(mockPool.filter((m) => mockSelected.has(m.id)));
	let isMock = $derived(data.entities.length === 0 && mockEntities.length >= 2);

	// In dev with mock data active, use mock entities for the grid
	let displayEntities = $derived(data.entities.length > 0 ? data.entities : mockEntities);

	function switchMockType(type: 'event' | 'company') {
		mockType = type;
		mockSelected = new Set();
	}

	function toggleMock(id: string) {
		const next = new Set(mockSelected);
		if (next.has(id)) next.delete(id);
		else next.add(id);
		mockSelected = next;
	}

	// For the grid: use displayEntities (real or mock) for categories
	const gridCategories = $derived(() => {
		const cats = new Set<string>();
		for (const entity of displayEntities) {
			for (const cr of entity.category_ratings) {
				cats.add(cr.category);
			}
		}
		const order = Object.keys(CATEGORY_LABELS);
		return [...cats].sort((a, b) => order.indexOf(a) - order.indexOf(b));
	});
</script>

<svelte:head>
	<title>Compare — RateMyHackathons</title>
</svelte:head>

<svelte:window onclick={handleWindowClick} />

<div class="mx-auto max-w-[1400px] px-6 py-24">
	<div use:fadeIn>
		<span class="text-[10px] uppercase tracking-[0.3em] text-dim">Compare</span>
		<h1 class="mt-2 font-display text-6xl italic sm:text-7xl">Side by Side</h1>
	</div>

	<!-- Type toggle -->
	<div class="mt-10 flex gap-6" use:fadeIn>
		<button
			class="text-xs uppercase tracking-[0.2em] transition-colors {searchType === 'event' ? 'text-text' : 'text-dim hover:text-muted'}"
			onclick={() => switchType('event')}
		>
			Events
		</button>
		<button
			class="text-xs uppercase tracking-[0.2em] transition-colors {searchType === 'company' ? 'text-text' : 'text-dim hover:text-muted'}"
			onclick={() => switchType('company')}
		>
			Companies
		</button>
	</div>

	<!-- Selected chips -->
	{#if chips.length > 0}
		<div class="mt-6 flex flex-wrap gap-3">
			{#each chips as chip}
				<span class="flex items-center gap-2 border border-border px-4 py-2 text-sm">
					{chip.name}
					<button
						onclick={() => removeEntity(chip.id)}
						class="text-dim transition-colors hover:text-text"
						aria-label="Remove {chip.name}"
					>&times;</button>
				</span>
			{/each}
		</div>
	{/if}

	<!-- Search input + dropdown -->
	{#if data.ids.length < 4}
		<div class="relative mt-6 max-w-lg" bind:this={wrapperEl} use:fadeIn>
			<div class="flex items-center border-b border-border transition-colors focus-within:border-text">
				<input
					bind:value={searchQuery}
					oninput={handleInput}
					type="text"
					placeholder="Search {searchType === 'event' ? 'events' : 'companies'} to compare{chips.length === 0 ? '...' : ''}"
					class="w-full bg-transparent py-3 text-sm text-text placeholder:text-dim focus:outline-none"
				/>
				{#if searching}
					<span class="text-[10px] text-dim">...</span>
				{/if}
			</div>

			{#if showDropdown}
				<div class="absolute left-0 right-0 top-full z-50 max-h-64 overflow-y-auto border border-border border-t-0 bg-surface">
					{#if searchResults.length > 0}
						{#each searchResults as result}
							<button
								class="flex w-full items-center justify-between border-b border-border px-4 py-3 text-left transition-colors hover:bg-elevated"
								onclick={() => addEntity(result.id, result.name)}
							>
								<span class="text-sm">{result.name}</span>
								<span class="flex items-center gap-2 text-xs text-muted">
									{#if result.avg_rating !== null}
										<ScoreBadge score={result.avg_rating} size="sm" />
									{/if}
									<span>{result.review_count} review{result.review_count !== 1 ? 's' : ''}</span>
								</span>
							</button>
						{/each}
					{:else}
						<p class="px-4 py-3 text-xs text-dim">No {searchType === 'event' ? 'events' : 'companies'} found.</p>
					{/if}
				</div>
			{/if}
		</div>
	{/if}

	<!-- Prompt to add more -->
	{#if data.ids.length === 1 && data.entities.length === 0}
		<p class="mt-4 text-xs text-muted">Add one more {searchType === 'event' ? 'event' : 'company'} to compare side-by-side.</p>
	{/if}

	{#if DEV && data.entities.length === 0 && chips.length === 0}
		<!-- Dev-only mock selector -->
		<div class="mt-8 border border-dashed border-yellow-600 p-4" use:fadeIn>
			<p class="mb-3 text-[10px] uppercase tracking-[0.3em] text-yellow-500">
				DEV — Backend down — Select mock {mockType}s to compare
			</p>

			<div class="mb-4 flex gap-4">
				<button
					class="text-xs uppercase tracking-[0.2em] transition-colors {mockType === 'event' ? 'text-yellow-500' : 'text-dim hover:text-muted'}"
					onclick={() => switchMockType('event')}
				>
					Events
				</button>
				<button
					class="text-xs uppercase tracking-[0.2em] transition-colors {mockType === 'company' ? 'text-yellow-500' : 'text-dim hover:text-muted'}"
					onclick={() => switchMockType('company')}
				>
					Companies
				</button>
			</div>

			<div class="flex flex-wrap gap-3">
				{#each mockPool as mock}
					<button
						onclick={() => toggleMock(mock.id)}
						class="border px-4 py-2 text-xs uppercase tracking-[0.15em] transition-colors {mockSelected.has(mock.id)
							? 'border-yellow-500 text-yellow-500'
							: 'border-border text-dim hover:border-muted hover:text-muted'}"
					>
						{mock.name} ({mock.avg_rating})
					</button>
				{/each}
			</div>
		</div>
	{/if}

	{#if isMock}
		<div class="mt-4 border border-yellow-600/30 bg-yellow-600/5 px-4 py-2 text-[10px] uppercase tracking-[0.2em] text-yellow-500">
			Showing mock data — backend is down
		</div>
	{/if}

	{#if displayEntities.length >= 2}
		<!-- Comparison Grid -->
		<div class="mt-12" use:fadeIn>
			<!-- Header Row -->
			<div class="grid border-b border-border" style="grid-template-columns: 200px repeat({displayEntities.length}, 1fr)">
				<div class="p-4"></div>
				{#each displayEntities as entity}
					<div class="p-4 text-center border-l border-border">
						<h2 class="font-display text-2xl italic">{entity.name}</h2>
						<div class="mt-3 flex justify-center">
							<ScoreBadge score={entity.avg_rating} size="lg" />
						</div>
						<p class="mt-2 text-xs text-muted">{entity.review_count} reviews</p>
						{#if entity.would_return_pct !== null}
							<p class="text-xs text-dim">{entity.would_return_pct.toFixed(0)}% would return</p>
						{/if}
					</div>
				{/each}
			</div>

			<!-- Category Rows -->
			{#each gridCategories() as category}
				{@const label = CATEGORY_LABELS[category as RatingCategory] ?? category}
				<div class="grid border-b border-border" style="grid-template-columns: 200px repeat({displayEntities.length}, 1fr)">
					<div class="p-4 flex items-center">
						<span class="text-[10px] uppercase tracking-[0.2em] text-muted">{label}</span>
					</div>
					{#each displayEntities as entity}
						{@const cat = entity.category_ratings.find((c) => c.category === category)}
						<div class="p-4 border-l border-border flex items-center justify-center">
							{#if cat}
								<div class="flex items-center gap-2">
									<div class="h-3 bg-elevated" style="width: 80px">
										<div class="h-full" style="width: {(cat.avg / 5) * 100}%; background-color: {scoreColor(cat.avg)}"></div>
									</div>
									<span class="font-display text-lg italic" style="color: {scoreColor(cat.avg)}">{cat.avg.toFixed(1)}</span>
								</div>
							{:else}
								<span class="text-xs text-dim">—</span>
							{/if}
						</div>
					{/each}
				</div>
			{/each}
		</div>

		<!-- Actions -->
		<div class="mt-8 flex gap-4">
			<button onclick={() => { nameCache.clear(); goto('/compare?type=' + searchType, { replaceState: true }); }} class="border border-border px-6 py-2 text-xs uppercase tracking-[0.2em] text-dim transition-colors hover:border-text hover:text-text">
				Reset
			</button>
			<button onclick={() => navigator.clipboard.writeText(window.location.href)} class="border border-border px-6 py-2 text-xs uppercase tracking-[0.2em] text-dim transition-colors hover:border-text hover:text-text">
				Copy Link
			</button>
		</div>
	{/if}
</div>
