<script lang="ts">
	import type { ReviewRef } from '$lib/types';

	let { review }: { review: ReviewRef } = $props();

	const dateStr = $derived(
		new Date(review.created_at).toLocaleDateString('en-US', { month: 'short', year: 'numeric' }).toUpperCase()
	);
</script>

<div class="border-l border-border py-6 pl-6 transition-colors hover:border-text">
	<!-- Rating as big number -->
	<div class="mb-4 flex items-baseline gap-3">
		<span class="font-display text-5xl italic">{review.rating}</span>
		<span class="text-[10px] tracking-[0.3em] text-dim">/ 5</span>
	</div>

	{#if review.title}
		<h4 class="mb-2 text-sm font-bold uppercase tracking-wide">{review.title}</h4>
	{/if}

	{#if review.body}
		<p class="mb-4 text-sm leading-relaxed text-muted">&ldquo;{review.body}&rdquo;</p>
	{/if}

	<div class="flex items-center gap-3 text-[10px] uppercase tracking-[0.3em] text-dim">
		<span>{review.username}</span>
		<span>&mdash;</span>
		<span>{dateStr}</span>
	</div>
</div>
