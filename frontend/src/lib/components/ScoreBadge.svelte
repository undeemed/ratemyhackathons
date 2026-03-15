<script lang="ts">
	import { scoreColor } from '$lib/types';

	let {
		score,
		size = 'md',
	}: {
		score: number | null;
		size?: 'sm' | 'md' | 'lg' | 'xl';
	} = $props();

	const sizeClasses: Record<string, string> = {
		sm: 'w-10 h-10 text-lg',
		md: 'w-14 h-14 text-2xl',
		lg: 'w-20 h-20 text-4xl',
		xl: 'w-28 h-28 text-6xl',
	};
</script>

{#if score !== null}
	<div
		class="flex items-center justify-center font-display italic {sizeClasses[size]}"
		style="background-color: {scoreColor(score)}"
	>
		<span class="text-black font-bold">{score.toFixed(1)}</span>
	</div>
{:else}
	<div
		class="flex items-center justify-center bg-elevated text-dim font-display italic {sizeClasses[size]}"
	>
		<span>N/A</span>
	</div>
{/if}
