<script lang="ts">
	import type { TagCount } from '$lib/types';
	import { voteTag } from '$lib/api';

	let { tags }: { tags: TagCount[] } = $props();

	let voteCounts = $state<Record<string, number>>({});
	let voting = $state<Record<string, boolean>>({});

	async function handleVote(tag: TagCount) {
		if (voting[tag.name]) return;
		voting[tag.name] = true;
		try {
			const res = await voteTag(tag.name);
			voteCounts[tag.name] = res.vote_count;
		} catch {
			// silently fail — tag voting is best-effort
		} finally {
			voting[tag.name] = false;
		}
	}

	function displayCount(tag: TagCount): number {
		return voteCounts[tag.name] ?? tag.count;
	}
</script>

{#if tags.length > 0}
	<div class="flex flex-wrap gap-2">
		{#each tags as tag}
			<button
				onclick={() => handleVote(tag)}
				disabled={voting[tag.name]}
				class="flex items-center gap-1 border border-border px-3 py-1 text-[10px] uppercase tracking-[0.15em] text-muted transition-colors hover:border-text hover:text-text disabled:opacity-50"
			>
				<span class="text-dim">&#9650;</span>
				{tag.name}
				<span class="ml-1 text-dim">({displayCount(tag)})</span>
			</button>
		{/each}
	</div>
{/if}
