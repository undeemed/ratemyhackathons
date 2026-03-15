<script lang="ts">
	import type { EventSummary } from '$lib/types';

	let { event, featured = false }: { event: EventSummary; featured?: boolean } = $props();

	const rating = $derived(event.avg_rating ? event.avg_rating.toFixed(1) : null);
	const dateStr = $derived(
		event.start_date
			? new Date(event.start_date).toLocaleDateString('en-US', { month: 'short', day: 'numeric' }).toUpperCase()
			: null
	);
</script>

<a
	href="/events/{event.id}"
	class="group block border border-border bg-bg transition-colors duration-300 hover:bg-elevated {featured ? '' : ''}"
>
	{#if event.image_url}
		<div class="overflow-hidden {featured ? 'aspect-[4/3]' : 'aspect-[16/9]'}">
			<img
				src={event.image_url}
				alt={event.name}
				class="h-full w-full object-cover grayscale transition-all duration-700 group-hover:scale-105 group-hover:grayscale-0"
			/>
		</div>
	{/if}

	<div class="p-5">
		<!-- Date + Rating row -->
		<div class="mb-3 flex items-center justify-between">
			{#if dateStr}
				<span class="text-[10px] tracking-[0.3em] text-dim">{dateStr}</span>
			{:else}
				<span class="text-[10px] tracking-[0.3em] text-dim">TBD</span>
			{/if}
			{#if rating}
				<span class="text-[10px] tracking-[0.3em] text-muted">{rating} / 5</span>
			{/if}
		</div>

		<!-- Title -->
		<h3 class="font-display text-xl italic leading-tight transition-colors group-hover:text-white sm:text-2xl {featured ? 'sm:text-3xl' : ''}">
			{event.name}
		</h3>

		<!-- Location -->
		{#if event.location}
			<p class="mt-2 text-xs text-muted">{event.location}</p>
		{/if}

		<!-- Companies -->
		{#if event.companies.length > 0}
			<div class="mt-4 flex gap-2">
				{#each event.companies.slice(0, 3) as company}
					<span class="border border-border px-2 py-0.5 text-[10px] uppercase tracking-[0.15em] text-dim transition-colors group-hover:border-muted group-hover:text-muted">
						{company.name}
					</span>
				{/each}
			</div>
		{/if}
	</div>
</a>
