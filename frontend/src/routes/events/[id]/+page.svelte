<script lang="ts">
	import ReviewCard from '$lib/components/ReviewCard.svelte';
	import { fadeIn, slideUp, staggerChildren } from '$lib/animations/gsap';
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

<div class="mx-auto max-w-[900px] px-6 py-24">
	<a href="/events" class="hover-line text-[10px] uppercase tracking-[0.3em] text-dim transition-colors hover:text-muted">&larr; Back</a>

	{#if event}
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

			{#if event.avg_rating}
				<div class="mt-8 border-l border-border pl-6">
					<span class="font-display text-6xl italic">{event.avg_rating.toFixed(1)}</span>
					<span class="ml-2 text-[10px] uppercase tracking-[0.3em] text-dim">/ 5 &mdash; {event.review_count} reviews</span>
				</div>
			{/if}
		</div>

		{#if event.description}
			<div class="mt-16 border-t border-border pt-8" use:slideUp>
				<h2 class="mb-4 text-[10px] uppercase tracking-[0.3em] text-dim">About</h2>
				<p class="whitespace-pre-line text-sm leading-relaxed text-muted">{event.description}</p>
			</div>
		{/if}

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

		<div class="mt-16 border-t border-border pt-8" use:fadeIn>
			<h2 class="mb-8 text-[10px] uppercase tracking-[0.3em] text-dim">Reviews ({event.review_count})</h2>
			{#if event.reviews.length > 0}
				<div class="space-y-0 divide-y divide-border" use:staggerChildren={{ stagger: 0.1 }}>
					{#each event.reviews as review (review.id)}
						<ReviewCard {review} />
					{/each}
				</div>
			{:else}
				<p class="py-12 text-center text-xs text-dim">No reviews yet.</p>
			{/if}
		</div>
	{:else}
		<div class="mt-24 text-center">
			<p class="font-display text-3xl italic text-muted">Event not found</p>
			<p class="mt-4 text-xs text-dim">The backend may not be running.</p>
		</div>
	{/if}
</div>
