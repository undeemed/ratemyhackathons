<script lang="ts">
	import type { TagCount } from '$lib/types';

	let { tags }: { tags: TagCount[] } = $props();
</script>

{#if tags.length > 0}
	<div class="flex flex-wrap gap-2">
		{#each tags as tag}
			<span class="border border-border px-3 py-1 text-[10px] uppercase tracking-[0.15em] text-muted transition-colors hover:border-text hover:text-text">
				{tag.name}
				<span class="ml-1 text-dim">({tag.count})</span>
			</span>
		{/each}
	</div>
{/if}
