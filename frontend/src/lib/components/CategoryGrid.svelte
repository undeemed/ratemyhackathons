<script lang="ts">
	import { scoreColor, CATEGORY_LABELS, type CategoryAvg, type RatingCategory } from '$lib/types';

	let { ratings }: { ratings: CategoryAvg[] } = $props();

	const sorted = $derived(
		[...ratings].sort((a, b) => {
			const order = Object.keys(CATEGORY_LABELS);
			return order.indexOf(a.category) - order.indexOf(b.category);
		})
	);
</script>

<div class="grid grid-cols-2 gap-x-8 gap-y-4">
	{#each sorted as { category, avg }}
		{@const label = CATEGORY_LABELS[category as RatingCategory] ?? category}
		<div class="flex items-center gap-3">
			<div class="w-full">
				<div class="flex items-center justify-between mb-1">
					<span class="text-[10px] uppercase tracking-[0.2em] text-muted">{label}</span>
					<span class="font-display text-lg italic" style="color: {scoreColor(avg)}">{avg.toFixed(1)}</span>
				</div>
				<div class="h-1 w-full bg-elevated">
					<div
						class="h-full transition-all duration-500"
						style="width: {(avg / 5) * 100}%; background-color: {scoreColor(avg)}"
					></div>
				</div>
			</div>
		</div>
	{/each}
</div>
