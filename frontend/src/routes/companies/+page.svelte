<script lang="ts">
	import { fadeIn, staggerChildren } from '$lib/animations/gsap';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>Companies — RateMyHackathons</title>
</svelte:head>

<div class="mx-auto max-w-[1400px] px-6 py-24">
	<div class="mb-16" use:fadeIn>
		<span class="text-[10px] uppercase tracking-[0.3em] text-dim">Directory</span>
		<h1 class="mt-2 font-display text-6xl italic sm:text-7xl">Companies</h1>
		<p class="mt-4 text-xs text-muted">{data.total} companies sponsoring hackathons</p>
		<div class="mt-6 h-px w-24 bg-border"></div>
	</div>

	<div class="grid gap-px border border-border sm:grid-cols-2 lg:grid-cols-3" use:staggerChildren={{ stagger: 0.04 }}>
		{#each data.companies as company (company.id)}
			<a
				href="/companies/{company.id}"
				class="group p-6 transition-colors hover:bg-elevated"
			>
				<h3 class="font-display text-2xl italic transition-colors group-hover:text-white">{company.name}</h3>
				{#if company.description}
					<p class="mt-2 line-clamp-2 text-xs text-muted">{company.description}</p>
				{/if}
			</a>
		{/each}
	</div>

	{#if data.companies.length === 0}
		<div class="border border-border py-24 text-center">
			<p class="font-display text-2xl italic text-muted">No companies yet.</p>
		</div>
	{/if}
</div>
