<script lang="ts">
	import ScoreBadge from '$lib/components/ScoreBadge.svelte';
	import { fadeIn } from '$lib/animations/gsap';
	import { scoreColor, CATEGORY_LABELS, type RatingCategory } from '$lib/types';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	let searchQuery = $state('');
	let searchType = $state<'event' | 'company'>(data.type ?? 'event');

	function handleAddEntity(e: SubmitEvent) {
		e.preventDefault();
		// Redirect to search to find entity, then come back
		if (searchQuery.trim()) {
			window.location.href = `/search?q=${encodeURIComponent(searchQuery.trim())}`;
		}
	}

	function handleReset() {
		window.location.href = '/compare';
	}

	function handleShare() {
		navigator.clipboard.writeText(window.location.href);
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
</script>

<svelte:head>
	<title>Compare — RateMyHackathons</title>
</svelte:head>

<div class="mx-auto max-w-[1400px] px-6 py-24">
	<div use:fadeIn>
		<span class="text-[10px] uppercase tracking-[0.3em] text-dim">Compare</span>
		<h1 class="mt-2 font-display text-6xl italic sm:text-7xl">Side by Side</h1>
	</div>

	{#if data.entities.length >= 2}
		<!-- Comparison Grid -->
		<div class="mt-12" use:fadeIn>
			<!-- Header Row -->
			<div class="grid border-b border-border" style="grid-template-columns: 200px repeat({data.entities.length}, 1fr)">
				<div class="p-4"></div>
				{#each data.entities as entity}
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
			{#each allCategories() as category}
				{@const label = CATEGORY_LABELS[category as RatingCategory] ?? category}
				<div class="grid border-b border-border" style="grid-template-columns: 200px repeat({data.entities.length}, 1fr)">
					<div class="p-4 flex items-center">
						<span class="text-[10px] uppercase tracking-[0.2em] text-muted">{label}</span>
					</div>
					{#each data.entities as entity}
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
			<button onclick={handleReset} class="border border-border px-6 py-2 text-xs uppercase tracking-[0.2em] text-dim transition-colors hover:border-text hover:text-text">
				Reset
			</button>
			<button onclick={handleShare} class="border border-border px-6 py-2 text-xs uppercase tracking-[0.2em] text-dim transition-colors hover:border-text hover:text-text">
				Copy Link
			</button>
		</div>
	{:else}
		<!-- Empty state: prompt to search for entities -->
		<div class="mt-12 border border-border p-8" use:fadeIn>
			<p class="mb-6 text-sm text-muted">
				{#if data.ids.length === 1}
					Search for a second {data.type} to compare.
				{:else}
					Search for two {data.type}s to compare side-by-side.
				{/if}
			</p>

			<div class="mb-6 flex gap-4">
				<button
					class="text-xs uppercase tracking-[0.2em] transition-colors {searchType === 'event' ? 'text-text' : 'text-dim hover:text-muted'}"
					onclick={() => { searchType = 'event'; }}
				>
					Events
				</button>
				<button
					class="text-xs uppercase tracking-[0.2em] transition-colors {searchType === 'company' ? 'text-text' : 'text-dim hover:text-muted'}"
					onclick={() => { searchType = 'company'; }}
				>
					Companies
				</button>
			</div>

			<form onsubmit={handleAddEntity}>
				<div class="flex items-center border-b border-border transition-colors focus-within:border-text">
					<input
						bind:value={searchQuery}
						type="text"
						placeholder="Search to find and compare..."
						class="w-full bg-transparent py-3 text-sm text-text placeholder:text-dim focus:outline-none"
					/>
				</div>
			</form>
		</div>
	{/if}
</div>
