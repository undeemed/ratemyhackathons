<script lang="ts">
	import { scoreColor, SCORE_LABELS } from '$lib/types';

	let {
		value = $bindable(0),
		label,
	}: {
		value: number;
		label: string;
	} = $props();

	const displayLabel = $derived(value > 0 ? SCORE_LABELS[value] : 'Not rated');
	const color = $derived(value > 0 ? scoreColor(value) : '#555');
</script>

<div class="space-y-2">
	<div class="flex items-center justify-between">
		<span class="text-[10px] uppercase tracking-[0.2em] text-muted">{label}</span>
		<span class="text-xs" style="color: {color}">{displayLabel}</span>
	</div>
	<div class="flex gap-1">
		{#each [1, 2, 3, 4, 5] as n}
			<button
				type="button"
				class="h-8 flex-1 border transition-all duration-200 text-[10px] uppercase tracking-wider
					{value >= n ? 'text-black font-bold' : 'border-border text-dim hover:border-muted'}"
				style={value >= n ? `background-color: ${scoreColor(n)}; border-color: ${scoreColor(n)}` : ''}
				onclick={() => { value = value === n ? 0 : n; }}
			>
				{n}
			</button>
		{/each}
	</div>
</div>
