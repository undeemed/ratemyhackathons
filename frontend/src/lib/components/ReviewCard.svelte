<script lang="ts">
	import { scoreColor, CATEGORY_LABELS, type ReviewRef, type RatingCategory } from '$lib/types';

	let { review }: { review: ReviewRef } = $props();

	const dateStr = $derived(
		new Date(review.created_at).toLocaleDateString('en-US', { month: 'short', year: 'numeric' }).toUpperCase()
	);

	const sortedCategories = $derived(
		(review.category_ratings ?? []).sort((a, b) => {
			const order = Object.keys(CATEGORY_LABELS);
			return order.indexOf(a.category) - order.indexOf(b.category);
		})
	);
</script>

<div class="border-l border-border py-6 pl-6 transition-colors hover:border-text">
	<div class="mb-4 flex items-baseline gap-3">
		<span class="font-display text-5xl italic" style="color: {scoreColor(review.rating)}">{review.rating}</span>
		<span class="text-[10px] tracking-[0.3em] text-dim">/ 5</span>
		{#if review.would_return !== null}
			<span class="ml-auto border border-border px-2 py-1 text-[9px] uppercase tracking-[0.15em] {review.would_return ? 'text-score-green border-score-green' : 'text-score-red border-score-red'}">
				{review.would_return ? 'Would return' : 'Would not return'}
			</span>
		{/if}
	</div>

	{#if review.title}
		<h4 class="mb-2 text-sm font-bold uppercase tracking-wide">{review.title}</h4>
	{/if}

	{#if review.body}
		<p class="mb-4 text-sm leading-relaxed text-muted">&ldquo;{review.body}&rdquo;</p>
	{/if}

	{#if sortedCategories.length > 0}
		<div class="mb-4 grid grid-cols-2 gap-x-6 gap-y-1 sm:grid-cols-5">
			{#each sortedCategories as { category, score }}
				{@const label = CATEGORY_LABELS[category as RatingCategory] ?? category}
				<div class="flex items-center justify-between gap-2">
					<span class="text-[9px] uppercase tracking-[0.1em] text-dim">{label}</span>
					<span class="text-xs font-bold" style="color: {scoreColor(score)}">{score}</span>
				</div>
			{/each}
		</div>
	{/if}

	<div class="flex items-center gap-3 text-[10px] uppercase tracking-[0.3em] text-dim">
		<span>{review.username}</span>
		<span>&mdash;</span>
		<span>{dateStr}</span>
	</div>
</div>
