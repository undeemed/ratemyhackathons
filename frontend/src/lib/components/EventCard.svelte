<script lang="ts">
	import type { EventSummary } from '$lib/types';

	let { event, featured = false, stretch = false }: { event: EventSummary; featured?: boolean; stretch?: boolean } = $props();

	const rating = $derived(event.avg_rating ? event.avg_rating.toFixed(1) : null);
	const dateStr = $derived(
		event.start_date
			? new Date(event.start_date).toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' }).toUpperCase()
			: null
	);
	const reviewLabel = $derived(
		event.review_count ? `${event.review_count} review${event.review_count !== 1 ? 's' : ''}` : null
	);
</script>

<a
	href="/events/{event.id}"
	class="group relative flex flex-col overflow-hidden border border-border bg-bg transition-all duration-500 hover:border-accent hover:bg-surface {featured ? 'row-span-2' : ''} {stretch ? 'flex-1' : ''}"
>
	<!-- Decorative corner accent -->
	<div class="absolute right-0 top-0 h-12 w-12 translate-x-6 -translate-y-6 rotate-45 bg-border transition-all duration-500 group-hover:bg-accent"></div>

	{#if event.image_url}
		<div class="overflow-hidden {featured ? 'aspect-[4/3]' : 'aspect-[16/9]'}">
			<img
				src={event.image_url}
				alt={event.name}
				class="h-full w-full object-cover grayscale transition-all duration-700 group-hover:scale-105 group-hover:grayscale-0"
			/>
		</div>
	{/if}

	<div class="flex flex-1 flex-col {featured ? 'p-8' : 'p-6'}">
		<!-- Top row: date + location -->
		<div class="flex items-center gap-3">
			{#if dateStr}
				<span class="text-[10px] tracking-[0.3em] text-dim">{dateStr}</span>
			{/if}
			{#if event.location && dateStr}
				<span class="text-dim">·</span>
			{/if}
			{#if event.location}
				<span class="text-[10px] tracking-[0.3em] text-dim">{event.location.toUpperCase()}</span>
			{/if}
		</div>

		<!-- Title -->
		<h3 class="mt-3 font-display italic leading-[1.1] tracking-tight transition-colors group-hover:text-white {featured ? 'text-4xl sm:text-5xl' : 'text-2xl sm:text-3xl'}">
			{event.name}
		</h3>

		<!-- Description (featured only) -->
		{#if featured && event.description}
			<p class="mt-3 text-xs leading-relaxed text-muted">{event.description}</p>
		{/if}

		<!-- Bottom row: rating + companies -->
		<div class="mt-auto flex items-end justify-between pt-5">
			<!-- Rating block -->
			{#if rating}
				<div class="flex items-baseline gap-2">
					<span class="font-display text-3xl italic text-accent transition-colors group-hover:text-white {featured ? 'text-4xl' : ''}">{rating}</span>
					<div class="flex flex-col">
						<span class="text-[9px] uppercase tracking-[0.2em] text-dim">/ 5.0</span>
						{#if reviewLabel}
							<span class="text-[9px] tracking-[0.15em] text-dim">{reviewLabel}</span>
						{/if}
					</div>
				</div>
			{/if}

			<!-- Company tags -->
			{#if event.companies.length > 0}
				<div class="flex gap-2">
					{#each event.companies.slice(0, featured ? 3 : 2) as company}
						<span class="border border-border px-2 py-1 text-[9px] uppercase tracking-[0.15em] text-dim transition-all duration-300 group-hover:border-accent group-hover:text-muted">
							{company.name}
						</span>
					{/each}
				</div>
			{/if}
		</div>
	</div>

	<!-- Bottom accent line -->
	<div class="h-[1px] w-0 bg-accent transition-all duration-500 group-hover:w-full"></div>
</a>
