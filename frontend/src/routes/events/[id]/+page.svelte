<script lang="ts">
	import ScoreBadge from '$lib/components/ScoreBadge.svelte';
	import CategoryGrid from '$lib/components/CategoryGrid.svelte';
	import RatingDistribution from '$lib/components/RatingDistribution.svelte';
	import TagPills from '$lib/components/TagPills.svelte';
	import ReviewCard from '$lib/components/ReviewCard.svelte';
	import { fadeIn, slideUp, staggerChildren } from '$lib/animations/gsap';
	import { scoreColor, scoreLabel } from '$lib/types';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	const event = $derived(data.event);

	const dateRange = $derived(() => {
		if (!event?.start_date) return null;
		const start = new Date(event.start_date).toLocaleDateString('en-US', { month: 'long', day: 'numeric', year: 'numeric' });
		if (!event.end_date) return start;
		const end = new Date(event.end_date).toLocaleDateString('en-US', { month: 'long', day: 'numeric', year: 'numeric' });
		return `${start} — ${end}`;
	});
</script>

<svelte:head>
	<title>{event?.name ?? 'Event'} — RateMyHackathons</title>
</svelte:head>

<div class="mx-auto max-w-[1100px] px-6 py-24">
	<a href="/events" class="hover-line text-[10px] uppercase tracking-[0.3em] text-dim transition-colors hover:text-muted">&larr; Back</a>

	{#if event}
		<!-- Header -->
		<div class="mt-12" use:fadeIn>
			{#if event.image_url}
				<img src={event.image_url} alt={event.name} class="mb-10 aspect-[21/9] w-full border border-border object-cover grayscale" />
			{/if}

			<h1 class="font-display text-5xl italic leading-[0.95] sm:text-6xl lg:text-7xl">{event.name}</h1>

			<div class="mt-6 flex flex-wrap gap-6 text-xs text-muted">
				{#if event.location}
					<span>{event.location}</span>
				{/if}
				{#if dateRange()}
					<span>{dateRange()}</span>
				{/if}
				{#if event.url}
					<a href={event.url} target="_blank" rel="noopener" class="hover-line text-text">Website &nearr;</a>
				{/if}
			</div>

			<div class="mt-6 flex flex-wrap items-center gap-4">
				<a href="/events/{event.id}/rate" class="border border-text px-6 py-2 text-xs uppercase tracking-[0.2em] text-text transition-colors hover:bg-text hover:text-black">
					Rate this event
				</a>
				<a href="/compare?type=event&ids={event.id}" class="border border-border px-6 py-2 text-xs uppercase tracking-[0.2em] text-dim transition-colors hover:border-text hover:text-text">
					Compare
				</a>
			</div>
		</div>

		<!-- Score Summary -->
		<div class="mt-16 border-t border-border pt-8" use:slideUp>
			<div class="flex flex-col items-center gap-4 sm:flex-row sm:gap-8">
				<ScoreBadge score={event.avg_rating} size="xl" />
				<div>
					<span class="text-[10px] uppercase tracking-[0.3em] text-dim">Overall Quality</span>
					{#if event.avg_rating}
						<p class="font-display text-xl italic" style="color: {scoreColor(event.avg_rating)}">
							{scoreLabel(event.avg_rating)}
						</p>
					{/if}
					<p class="text-xs text-muted">Based on {event.review_count} review{event.review_count !== 1 ? 's' : ''}</p>
					{#if event.would_return_pct !== null}
						<p class="mt-1 text-xs text-muted">{event.would_return_pct.toFixed(0)}% would attend again</p>
					{/if}
				</div>
			</div>
		</div>

		<!-- Category Ratings -->
		{#if event.category_ratings.length > 0}
			<div class="mt-12 border-t border-border pt-8" use:fadeIn>
				<h2 class="mb-6 text-[10px] uppercase tracking-[0.3em] text-dim">Category Ratings</h2>
				<CategoryGrid ratings={event.category_ratings} />
			</div>
		{/if}

		<!-- Tags -->
		{#if event.top_tags.length > 0}
			<div class="mt-12 border-t border-border pt-8" use:fadeIn>
				<h2 class="mb-4 text-[10px] uppercase tracking-[0.3em] text-dim">Top Tags</h2>
				<TagPills tags={event.top_tags} />
			</div>
		{/if}

		<!-- Rating Distribution -->
		{#if event.rating_distribution.length > 0}
			<div class="mt-12 border-t border-border pt-8" use:fadeIn>
				<h2 class="mb-4 text-[10px] uppercase tracking-[0.3em] text-dim">Rating Distribution</h2>
				<div class="max-w-md">
					<RatingDistribution distribution={event.rating_distribution} />
				</div>
			</div>
		{/if}

		<!-- About -->
		{#if event.description}
			<div class="mt-12 border-t border-border pt-8" use:slideUp>
				<h2 class="mb-4 text-[10px] uppercase tracking-[0.3em] text-dim">About</h2>
				<p class="whitespace-pre-line text-sm leading-relaxed text-muted">{event.description}</p>
			</div>
		{/if}

		<!-- Companies -->
		{#if event.companies.length > 0}
			<div class="mt-12 border-t border-border pt-8" use:fadeIn>
				<h2 class="mb-4 text-[10px] uppercase tracking-[0.3em] text-dim">Companies</h2>
				<div class="flex flex-wrap gap-2">
					{#each event.companies as company}
						<a
							href="/companies/{company.id}"
							class="border border-border px-4 py-2 text-xs uppercase tracking-[0.15em] text-muted transition-colors hover:border-text hover:text-text"
						>
							{company.name}
							{#if company.role}<span class="ml-1 text-dim">({company.role})</span>{/if}
						</a>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Reviews -->
		<div class="mt-16 border-t border-border pt-8" use:fadeIn>
			<h2 class="mb-8 text-[10px] uppercase tracking-[0.3em] text-dim">Reviews ({event.review_count})</h2>
			{#if event.reviews.length > 0}
				<div class="space-y-0 divide-y divide-border" use:staggerChildren={{ stagger: 0.1 }}>
					{#each event.reviews as review (review.id)}
						<ReviewCard {review} />
					{/each}
				</div>
			{:else}
				<p class="py-12 text-center text-xs text-dim">No reviews yet. Be the first to <a href="/events/{event.id}/rate" class="hover-line text-text">rate this event</a>.</p>
			{/if}
		</div>
	{:else}
		<div class="mt-24 text-center">
			<p class="font-display text-3xl italic text-muted">Event not found</p>
			<p class="mt-4 text-xs text-dim">The backend may not be running.</p>
		</div>
	{/if}
</div>
