<script lang="ts">
	import EventCard from '$lib/components/EventCard.svelte';
	import { fadeIn } from '$lib/animations/gsap';
	import type { PageData } from './$types';
	import { RATING_CATEGORIES, CATEGORY_LABELS, type RatingCategory } from '$lib/types';

	let { data }: { data: PageData } = $props();

	let view: 'list' | 'grid' = $state('list');

	type SortKey = 'date' | 'name' | 'rating' | RatingCategory;
	let sortBy: SortKey = $state('date');
	let sortDir: 'asc' | 'desc' = $state('desc');

	let search = $state('');

	function getCategoryAvg(event: (typeof data.events)[0], category: string): number {
		const cat = event.category_ratings?.find((c) => c.category === category);
		return cat?.avg ?? 0;
	}

	const filtered = $derived.by(() => {
		let events = data.events;

		if (search.trim()) {
			const q = search.toLowerCase().trim();
			events = events.filter(
				(e) =>
					e.name.toLowerCase().includes(q) ||
					e.location?.toLowerCase().includes(q),
			);
		}

		return [...events].sort((a, b) => {
			let cmp = 0;
			if (sortBy === 'date') {
				cmp = (a.start_date ?? '').localeCompare(b.start_date ?? '');
			} else if (sortBy === 'name') {
				cmp = a.name.localeCompare(b.name);
			} else if (sortBy === 'rating') {
				cmp = (a.avg_rating ?? 0) - (b.avg_rating ?? 0);
			} else {
				cmp = getCategoryAvg(a, sortBy) - getCategoryAvg(b, sortBy);
			}
			return sortDir === 'desc' ? -cmp : cmp;
		});
	});

	function handleSortChange(e: Event) {
		const key = (e.target as HTMLSelectElement).value as SortKey;
		if (sortBy === key) return;
		sortBy = key;
		sortDir = key === 'name' ? 'asc' : 'desc';
	}

	function toggleDir() {
		sortDir = sortDir === 'desc' ? 'asc' : 'desc';
	}

	function sortLabel(key: SortKey): string {
		if (key === 'date') return 'Date';
		if (key === 'name') return 'Name';
		if (key === 'rating') return 'Overall Rating';
		return CATEGORY_LABELS[key as RatingCategory];
	}

	function fmtDate(d: string | null) {
		if (!d) return '—';
		return new Date(d)
			.toLocaleDateString('en-US', {
				month: 'short',
				day: 'numeric',
				year: 'numeric',
			})
			.toUpperCase();
	}

	function ratingDisplay(avg: number | null): string {
		if (avg === null) return '—';
		return avg.toFixed(1);
	}

	function ratingColor(avg: number | null): string {
		if (avg === null) return 'text-dim';
		if (avg >= 4.0) return 'text-[#4caf50]';
		if (avg >= 3.0) return 'text-[#ffc107]';
		return 'text-[#ef5350]';
	}
</script>

<svelte:head>
	<title>Events — RateMyHackathons</title>
</svelte:head>

<div class="mx-auto max-w-[1400px] px-6 py-24">
	<!-- Header -->
	<div class="mb-12" use:fadeIn>
		<a
			href="/"
			class="mb-6 inline-block text-xs uppercase tracking-[0.2em] text-dim transition-colors hover:text-text"
			>&larr; Back</a
		>
		<h1 class="mt-2 font-display text-6xl italic sm:text-7xl">All events</h1>
		<p class="mt-4 text-xs text-muted">
			{data.total} hackathons and counting
		</p>
		<div class="mt-6 h-px w-24 bg-border"></div>
	</div>

	<!-- Toolbar -->
	<div
		class="mb-8 flex flex-col gap-4 border border-border bg-surface p-4 sm:flex-row sm:items-center sm:justify-between"
		use:fadeIn
	>
		<!-- Search -->
		<div class="relative flex-1 sm:max-w-sm">
			<input
				type="text"
				bind:value={search}
				placeholder="Search events..."
				class="w-full border border-border bg-bg px-3 py-2 text-xs tracking-wide text-text placeholder:text-dim focus:border-accent focus:outline-none"
			/>
			{#if search}
				<button
					onclick={() => (search = '')}
					class="absolute right-2 top-1/2 -translate-y-1/2 text-xs text-dim transition-colors hover:text-text"
				>
					×
				</button>
			{/if}
		</div>

		<div class="flex items-center gap-3">
			<!-- Sort -->
			<div class="flex items-center gap-1">
				<span class="mr-1 text-[10px] uppercase tracking-[0.2em] text-dim">Sort</span>
				<select
					value={sortBy}
					onchange={handleSortChange}
					class="appearance-none border border-border bg-bg px-2 py-1 pr-6 text-[10px] uppercase tracking-[0.15em] text-text focus:border-accent focus:outline-none"
					style="background-image: url('data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 width=%2210%22 height=%226%22><path d=%22M0 0l5 6 5-6z%22 fill=%22%23555%22/></svg>'); background-repeat: no-repeat; background-position: right 6px center;"
				>
					<optgroup label="General">
						<option value="date">Date</option>
						<option value="name">Name</option>
						<option value="rating">Overall Rating</option>
					</optgroup>
					<optgroup label="Category">
						{#each RATING_CATEGORIES as cat}
							<option value={cat}>{CATEGORY_LABELS[cat]}</option>
						{/each}
					</optgroup>
				</select>
				<button
					onclick={toggleDir}
					class="border border-border px-2 py-1 text-[10px] text-accent transition-all hover:bg-elevated"
					title="{sortDir === 'desc' ? 'Descending' : 'Ascending'} — click to flip"
				>
					{sortDir === 'desc' ? '↓' : '↑'}
				</button>
			</div>

			<!-- View toggle -->
			<div class="flex border border-border">
				<button
					onclick={() => (view = 'list')}
					class="px-2 py-1 text-xs transition-all {view === 'list'
						? 'bg-elevated text-text'
						: 'text-dim hover:text-muted'}"
					title="List view"
				>
					≡
				</button>
				<button
					onclick={() => (view = 'grid')}
					class="border-l border-border px-2 py-1 text-xs transition-all {view ===
					'grid'
						? 'bg-elevated text-text'
						: 'text-dim hover:text-muted'}"
					title="Grid view"
				>
					⊞
				</button>
			</div>
		</div>
	</div>

	<!-- Results count -->
	{#if search.trim()}
		<p class="mb-4 text-[10px] uppercase tracking-[0.3em] text-dim">
			{filtered.length} result{filtered.length !== 1 ? 's' : ''}
		</p>
	{/if}

	<!-- Grid view -->
	{#if view === 'grid'}
		<div class="grid gap-px border border-border sm:grid-cols-2 lg:grid-cols-3">
			{#each filtered as event (event.id)}
				<EventCard {event} />
			{/each}
		</div>
	{:else}
		<!-- List view -->
		<div class="flex flex-col">
			<!-- Column headers -->
			{#if filtered.length > 0}
				<div class="mb-2 flex items-center px-4 text-[10px] uppercase tracking-[0.3em] text-dim">
					<span class="w-16 shrink-0 text-center">{sortLabel(sortBy)}</span>
					<span class="flex-1 pl-4">Event</span>
					<span class="hidden w-40 shrink-0 text-right sm:block">Location</span>
				</div>
			{/if}
			<div class="border-t border-border">
				{#each filtered as event (event.id)}
					{@const displayRating = sortBy === 'date' || sortBy === 'name'
						? event.avg_rating
						: sortBy === 'rating'
							? event.avg_rating
							: getCategoryAvg(event, sortBy) || null}
					<a
						href="/events/{event.id}"
						class="group flex items-center border-b border-border px-4 py-4 transition-colors hover:bg-elevated"
					>
						<!-- Rating -->
						<span class="w-16 shrink-0 text-center font-mono text-lg font-bold {ratingColor(displayRating)}">
							{ratingDisplay(displayRating)}
						</span>

						<!-- Name + date -->
						<div class="flex-1 min-w-0 pl-4">
							<h3 class="font-display text-xl italic transition-colors group-hover:text-white">{event.name}</h3>
							<p class="mt-0.5 text-[10px] tracking-[0.3em] text-dim">
								{fmtDate(event.start_date)}
								{#if event.review_count > 0}
									<span class="ml-2">{event.review_count} review{event.review_count !== 1 ? 's' : ''}</span>
								{/if}
							</p>
						</div>

						<!-- Location -->
						<span class="hidden w-40 shrink-0 text-right font-mono text-xs text-muted sm:block">
							{event.location?.toUpperCase() ?? '—'}
						</span>
					</a>
				{/each}
			</div>
		</div>
	{/if}

	{#if filtered.length === 0}
		<div class="border border-border py-24 text-center">
			<p class="font-display text-2xl italic text-muted">
				{search.trim() ? 'No matches found.' : 'Nothing here yet.'}
			</p>
		</div>
	{/if}
</div>
