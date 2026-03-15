<script lang="ts">
	import { scoreColor, SCORE_LABELS, type RatingDistributionEntry } from '$lib/types';

	let { distribution }: { distribution: RatingDistributionEntry[] } = $props();

	const maxCount = $derived(Math.max(...distribution.map((d) => d.count), 1));
	const totalReviews = $derived(distribution.reduce((sum, d) => sum + d.count, 0));

	const rows = $derived(
		[5, 4, 3, 2, 1].map((rating) => {
			const entry = distribution.find((d) => d.rating === rating);
			return {
				rating,
				count: entry?.count ?? 0,
				label: SCORE_LABELS[rating],
			};
		})
	);
</script>

<div class="space-y-2">
	{#each rows as row}
		<div class="flex items-center gap-3">
			<span class="w-16 text-right text-[10px] uppercase tracking-[0.15em] text-muted">{row.label}</span>
			<div class="flex-1 h-5 bg-elevated relative">
				<div
					class="h-full transition-all duration-500"
					style="width: {totalReviews > 0 ? (row.count / maxCount) * 100 : 0}%; background-color: {scoreColor(row.rating)}"
				></div>
			</div>
			<span class="w-8 text-right text-xs text-dim">{row.count}</span>
		</div>
	{/each}
</div>
