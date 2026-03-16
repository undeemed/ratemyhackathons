<script lang="ts">
	import { fadeIn } from '$lib/animations/gsap';
	import { RATING_CATEGORIES, CATEGORY_LABELS, type RatingCategory } from '$lib/types';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	type SortKey = 'name' | 'rating' | 'events' | 'latest_event' | RatingCategory;
	let sortBy: SortKey = $state('name');
	let sortDir: 'asc' | 'desc' = $state('asc');

	let search = $state('');

	function getCategoryAvg(company: (typeof data.companies)[0], cat: string): number | null {
		const entry = company.category_ratings.find((c) => c.category === cat);
		return entry ? entry.avg : null;
	}

	const filtered = $derived.by(() => {
		let companies = data.companies;

		if (search.trim()) {
			const q = search.toLowerCase().trim();
			companies = companies.filter(
				(c) =>
					c.name.toLowerCase().includes(q) ||
					c.description?.toLowerCase().includes(q),
			);
		}

		return [...companies].sort((a, b) => {
			let cmp = 0;
			if (sortBy === 'name') {
				cmp = a.name.localeCompare(b.name);
			} else if (sortBy === 'rating') {
				cmp = (a.avg_rating ?? 0) - (b.avg_rating ?? 0);
			} else if (sortBy === 'events') {
				cmp = a.event_count - b.event_count;
			} else if (sortBy === 'latest_event') {
				cmp = (a.latest_event_date ?? '').localeCompare(b.latest_event_date ?? '');
			} else {
				cmp = (getCategoryAvg(a, sortBy) ?? 0) - (getCategoryAvg(b, sortBy) ?? 0);
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

	const isCategorySort = $derived(RATING_CATEGORIES.includes(sortBy as RatingCategory));
</script>

<svelte:head>
	<title>Companies — RateMyHackathons</title>
</svelte:head>

<div class="mx-auto max-w-[1400px] px-6 py-24">
	<!-- Header -->
	<div class="mb-12" use:fadeIn>
		<a
			href="/"
			class="mb-6 inline-block text-xs uppercase tracking-[0.2em] text-dim transition-colors hover:text-text"
			>&larr; Back</a
		>
		<h1 class="mt-2 font-display text-6xl italic sm:text-7xl">Companies</h1>
		<p class="mt-4 text-xs text-muted">
			{data.total} companies sponsoring hackathons
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
				placeholder="Search companies..."
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
					<option value="name">Name</option>
					<option value="rating">Overall Rating</option>
					<option value="events">Events Hosted</option>
					<option value="latest_event">Most Recent Event</option>
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
	</div>

	<!-- Results count -->
	{#if search.trim()}
		<p class="mb-4 text-[10px] uppercase tracking-[0.3em] text-dim">
			{filtered.length} result{filtered.length !== 1 ? 's' : ''}
		</p>
	{/if}

	<!-- Column headers -->
	{#if filtered.length > 0}
		<div class="mb-2 flex items-center px-4 text-[10px] uppercase tracking-[0.3em] text-dim">
			<span class="w-16 shrink-0 text-center">{isCategorySort ? CATEGORY_LABELS[sortBy as RatingCategory] : 'Rating'}</span>
			<span class="flex-1 pl-4">Company</span>
			<span class="w-28 shrink-0 text-right">Events Hosted</span>
		</div>
	{/if}

	<!-- Company list -->
	<div class="border-t border-border">
		{#each filtered as company (company.id)}
			{@const displayRating = isCategorySort ? getCategoryAvg(company, sortBy) : company.avg_rating}
			<a
				href="/companies/{company.id}"
				class="group flex items-center border-b border-border px-4 py-4 transition-colors hover:bg-elevated"
			>
				<!-- Rating -->
				<span class="w-16 shrink-0 text-center font-mono text-lg font-bold {ratingColor(displayRating)}">
					{ratingDisplay(displayRating)}
				</span>

				<!-- Name + description -->
				<div class="flex-1 min-w-0 pl-4">
					<h3 class="font-display text-xl italic transition-colors group-hover:text-white">{company.name}</h3>
					{#if company.description}
						<p class="mt-0.5 truncate text-xs text-muted">{company.description}</p>
					{/if}
				</div>

				<!-- Events hosted -->
				<span class="w-28 shrink-0 text-right font-mono text-sm text-muted">
					{company.event_count} {company.event_count === 1 ? 'event' : 'events'}
				</span>
			</a>
		{/each}
	</div>

	{#if filtered.length === 0}
		<div class="border border-border py-24 text-center">
			<p class="font-display text-2xl italic text-muted">
				{search.trim() ? 'No matches found.' : 'No companies yet.'}
			</p>
		</div>
	{/if}
</div>
