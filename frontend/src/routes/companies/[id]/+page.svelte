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

	const company = $derived(data.company);
</script>

<svelte:head>
	<title>{company?.name ?? 'Company'} — RateMyHackathons</title>
</svelte:head>

<div class="mx-auto max-w-[1100px] px-6 py-24">
	<a href="/companies" class="hover-line text-[10px] uppercase tracking-[0.3em] text-dim transition-colors hover:text-muted">&larr; Back</a>

	{#if company}
		<!-- Header -->
		<div class="mt-12" use:fadeIn>
			<h1 class="font-display text-6xl italic sm:text-7xl">{company.name}</h1>
			{#if company.description}
				<p class="mt-4 max-w-lg text-sm text-muted">{company.description}</p>
			{/if}
			<div class="mt-6 flex flex-wrap items-center gap-4">
				{#if company.website}
					<a href={company.website} target="_blank" rel="noopener" class="hover-line text-xs text-muted transition-colors hover:text-text">
						{company.website} &nearr;
					</a>
				{/if}
				<a href="/companies/{company.id}/rate" class="border border-text px-6 py-2 text-xs uppercase tracking-[0.2em] text-text transition-colors hover:bg-text hover:text-black">
					Rate this company
				</a>
				<a href="/compare?type=company&ids={company.id}" class="border border-border px-6 py-2 text-xs uppercase tracking-[0.2em] text-dim transition-colors hover:border-text hover:text-text">
					Compare
				</a>
			</div>
		</div>

		<!-- Score Summary -->
		<div class="mt-16 border-t border-border pt-8" use:slideUp>
			<div class="flex flex-col items-center gap-4 sm:flex-row sm:gap-8">
				<ScoreBadge score={company.avg_rating} size="xl" />
				<div>
					<span class="text-[10px] uppercase tracking-[0.3em] text-dim">Overall Quality</span>
					{#if company.avg_rating}
						<p class="font-display text-xl italic" style="color: {scoreColor(company.avg_rating)}">
							{scoreLabel(company.avg_rating)}
						</p>
					{/if}
					<p class="text-xs text-muted">Based on {company.review_count} review{company.review_count !== 1 ? 's' : ''}</p>
					{#if company.would_return_pct !== null}
						<p class="mt-1 text-xs text-muted">{company.would_return_pct.toFixed(0)}% would attend again</p>
					{/if}
				</div>
			</div>
		</div>

		<!-- Category Ratings -->
		{#if company.category_ratings.length > 0}
			<div class="mt-12 border-t border-border pt-8" use:fadeIn>
				<h2 class="mb-6 text-[10px] uppercase tracking-[0.3em] text-dim">Category Ratings</h2>
				<CategoryGrid ratings={company.category_ratings} />
			</div>
		{/if}

		<!-- Tags -->
		{#if company.top_tags.length > 0}
			<div class="mt-12 border-t border-border pt-8" use:fadeIn>
				<h2 class="mb-4 text-[10px] uppercase tracking-[0.3em] text-dim">Top Tags</h2>
				<TagPills tags={company.top_tags} />
			</div>
		{/if}

		<!-- Rating Distribution -->
		{#if company.rating_distribution.length > 0}
			<div class="mt-12 border-t border-border pt-8" use:fadeIn>
				<h2 class="mb-4 text-[10px] uppercase tracking-[0.3em] text-dim">Rating Distribution</h2>
				<div class="max-w-md">
					<RatingDistribution distribution={company.rating_distribution} />
				</div>
			</div>
		{/if}

		<!-- Events -->
		{#if company.events.length > 0}
			<div class="mt-12 border-t border-border pt-8" use:fadeIn>
				<h2 class="mb-6 text-[10px] uppercase tracking-[0.3em] text-dim">Events ({company.events.length})</h2>
				<div class="space-y-0 divide-y divide-border">
					{#each company.events as event}
						<a href="/events/{event.id}" class="group flex items-center justify-between p-4 transition-colors hover:bg-elevated">
							<div>
								<h3 class="font-display text-xl italic transition-colors group-hover:text-white">{event.name}</h3>
								<div class="mt-1 flex gap-3 text-[10px] text-dim">
									{#if event.role}
										<span class="uppercase tracking-[0.15em]">{event.role}</span>
									{/if}
									{#if event.start_date}
										<span>{new Date(event.start_date).toLocaleDateString('en-US', { month: 'short', year: 'numeric' })}</span>
									{/if}
								</div>
							</div>
							{#if event.avg_rating}
								<span class="font-display text-2xl italic" style="color: {scoreColor(event.avg_rating)}">{event.avg_rating.toFixed(1)}</span>
							{/if}
						</a>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Reviews -->
		<div class="mt-16 border-t border-border pt-8" use:fadeIn>
			<h2 class="mb-8 text-[10px] uppercase tracking-[0.3em] text-dim">Reviews ({company.review_count})</h2>
			{#if company.reviews.length > 0}
				<div class="space-y-0 divide-y divide-border" use:staggerChildren={{ stagger: 0.1 }}>
					{#each company.reviews as review (review.id)}
						<ReviewCard {review} />
					{/each}
				</div>
			{:else}
				<p class="py-12 text-center text-xs text-dim">No reviews yet. Be the first to <a href="/companies/{company.id}/rate" class="hover-line text-text">rate this company</a>.</p>
			{/if}
		</div>
	{:else}
		<div class="mt-24 text-center">
			<p class="font-display text-3xl italic text-muted">Company not found</p>
			<p class="mt-4 text-xs text-dim">The backend may not be running.</p>
		</div>
	{/if}
</div>
